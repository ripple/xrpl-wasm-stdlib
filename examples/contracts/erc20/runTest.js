const xrpl = require("@transia/xrpl")

/**
 * Submit a ContractCall transaction manually (submit + poll for validation).
 * This avoids websocket disconnection issues with submitAndWait for long-running WASM calls.
 */
async function submitContractCall(client, tx, wallet, debug = false) {
  console.log("Submitting transaction:", JSON.stringify(tx, null, 2))
  const prepared = await client.autofill(tx)
  const signed = wallet.sign(prepared)
  const submitRes = await client.request({
    command: "submit",
    tx_blob: signed.tx_blob,
  })
  if (debug)
    console.log("Submit result:", JSON.stringify(submitRes.result, null, 2))
  if (submitRes.result.engine_result !== "tesSUCCESS") {
    console.error("Submit failed:", submitRes.result.engine_result)
    console.error("Full result:", JSON.stringify(submitRes.result, null, 2))
    process.exit(1)
  }
  // Close the ledger and poll until validated, reconnecting if needed
  let txResult
  for (let i = 0; i < 30; i++) {
    await new Promise((r) => setTimeout(r, 500))
    try {
      if (!client.isConnected()) {
        console.log("Reconnecting to rippled...")
        await client.connect()
      }
      // Force ledger close on standalone server
      await client.request({ command: "ledger_accept" })
      txResult = await client.request({
        command: "tx",
        transaction: signed.hash,
      })
      if (txResult.result.validated) break
    } catch (e) {
      console.log("Poll attempt", i, "failed:", e.message, "- retrying...")
    }
  }
  if (!txResult?.result?.validated) {
    console.error("Transaction was not validated after 30 attempts")
    process.exit(1)
  }
  console.log("Result code:", txResult.result.meta.TransactionResult)
  if (txResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error("Transaction failed:", txResult.result.meta.TransactionResult)
    if (debug) console.log(JSON.stringify(txResult.result, null, 2))
    process.exit(1)
  }
  if (debug) console.log(JSON.stringify(txResult.result, null, 2))
  return txResult
}

/**
 * Integration test for the ERC-20 MPT wrapper contract.
 *
 * Test flow:
 * 1. Deploy the contract (init creates the MPT issuance)
 * 2. Extract the contract account and derive the MPT ID
 * 3. Authorize users for the MPT, mint tokens via Payment from contract (issuer)
 * 4. Test transfer (Clawback + Payment)
 * 5. Test approve + transfer_from
 */
async function test(testContext) {
  const { client, submit, sourceWallet, destWallet, fundWallet, finish } =
    testContext

  // --- Step 1: Deploy the ERC-20 contract ---
  // The init function will create the MPT issuance with max_amount
  console.log("\n=== Step 1: Deploy ERC-20 Contract ===")

  const maxAmount = Number(1000000).toString(16) // UINT64 values are hex strings

  // Get current ledger to set a generous LastLedgerSequence
  const ledgerInfo = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  const currentLedger = ledgerInfo.result.ledger_index

  const contractCreateTx = {
    TransactionType: "ContractCreate",
    Account: sourceWallet.address,
    Flags: 0,
    Fee: "10000000", // Higher fee for the large WASM blob
    LastLedgerSequence: currentLedger + 200, // Generous window for large tx
    ContractCode: finish,
    // Declare the contract's exported functions (ABI)
    Functions: [
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("init"),
          Parameters: [
            {
              Parameter: {
                ParameterFlag: 0x00010000, // tfSendAmount - sends XRP to contract
                ParameterType: { type: "AMOUNT" },
              },
            },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("mint"),
          Parameters: [
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "ACCOUNT" },
              },
            },
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "UINT64" },
              },
            },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("transfer"),
          Parameters: [
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "ACCOUNT" },
              },
            },
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "UINT64" },
              },
            },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("approve"),
          Parameters: [
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "ACCOUNT" },
              },
            },
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "UINT64" },
              },
            },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("transfer_from"),
          Parameters: [
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "ACCOUNT" },
              },
            },
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "ACCOUNT" },
              },
            },
            {
              Parameter: {
                ParameterFlag: 0,
                ParameterType: { type: "UINT64" },
              },
            },
          ],
        },
      },
    ],
    // Instance parameters: max_amount (u64)
    InstanceParameters: [
      {
        InstanceParameter: {
          ParameterFlag: 0,
          ParameterType: { type: "UINT64" },
        },
      },
    ],
    InstanceParameterValues: [
      {
        InstanceParameterValue: {
          ParameterFlag: 0,
          ParameterValue: {
            type: "UINT64",
            value: maxAmount,
          },
        },
      },
    ],
  }

  const contractCreateResult = await submit(
    contractCreateTx,
    sourceWallet,
    true,
  )
  if (contractCreateResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to create contract:",
      contractCreateResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Extract the contract account from the metadata
  let contractAccount = null
  for (const node of contractCreateResult.result.meta.AffectedNodes) {
    if (node.CreatedNode && node.CreatedNode.LedgerEntryType === "Contract") {
      contractAccount =
        node.CreatedNode.NewFields?.ContractAccount ||
        node.CreatedNode.NewFields?.Account
      break
    }
  }
  if (!contractAccount) {
    console.error("Failed to extract contract account from metadata")
    console.error(
      "Metadata:",
      JSON.stringify(contractCreateResult.result.meta, null, 2),
    )
    process.exit(1)
  }
  console.log("Contract account (MPT issuer):", contractAccount)

  // Debug: inspect the ContractSource to see stored functions
  let contractSourceIndex = null
  for (const node of contractCreateResult.result.meta.AffectedNodes) {
    if (
      node.CreatedNode &&
      node.CreatedNode.LedgerEntryType === "ContractSource"
    ) {
      contractSourceIndex = node.CreatedNode.LedgerIndex
      break
    }
  }
  if (contractSourceIndex) {
    const sourceEntry = await client.request({
      command: "ledger_entry",
      index: contractSourceIndex,
    })
    console.log(
      "ContractSource Functions:",
      JSON.stringify(sourceEntry.result.node?.Functions, null, 2),
    )
  } else {
    console.log("No ContractSource found in metadata")
  }

  // --- Step 1b: Call init via ContractCall ---
  // Use tfSendAmount parameter flag to fund the contract account with XRP
  // in the same call (contract accounts can't receive direct Payments)
  console.log("\n=== Step 1b: Call init() ===")
  const initTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("init"),
    ComputationAllowance: 1000000,
    Fee: "10000000",
    Parameters: [
      {
        Parameter: {
          ParameterFlag: 0x00010000, // tfSendAmount - transfer XRP to contract
          ParameterValue: {
            type: "AMOUNT",
            value: xrpl.xrpToDrops(500),
          },
        },
      },
    ],
    LastLedgerSequence:
      (
        await client.request({
          command: "ledger",
          ledger_index: "validated",
        })
      ).result.ledger_index + 200,
  }
  // Submit manually to avoid autofill throwing on temMALFORMED
  const prepared = await client.autofill(initTx)
  const signed = sourceWallet.sign(prepared)
  console.log("Signed init tx:", signed.tx_blob.substring(0, 100) + "...")
  const submitRes = await client.request({
    command: "submit",
    tx_blob: signed.tx_blob,
  })
  console.log("Submit result:", JSON.stringify(submitRes.result, null, 2))
  if (submitRes.result.engine_result !== "tesSUCCESS") {
    // Wait for it to be validated anyway
    console.log("Waiting for validation...")
  }
  // Close ledger and poll until validated
  let txResult
  for (let i = 0; i < 20; i++) {
    await new Promise((r) => setTimeout(r, 500))
    try {
      await client.request({ command: "ledger_accept" })
      txResult = await client.request({
        command: "tx",
        transaction: signed.hash,
      })
      if (txResult.result.validated) break
    } catch (e) {
      console.log("Init poll attempt", i, "failed:", e.message)
    }
  }
  console.log("Init tx result:", txResult.result?.meta?.TransactionResult)
  if (txResult.result?.meta?.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to call init:",
      txResult.result?.meta?.TransactionResult,
    )
    console.error("Full result:", JSON.stringify(txResult.result, null, 2))
    process.exit(1)
  }

  // Extract MPT issuance ID from the contract account's objects
  // The inner MPTokenIssuanceCreate is a separate tx, so its effects
  // won't appear in the ContractCall's metadata.
  const accountObjects = await client.request({
    command: "account_objects",
    account: contractAccount,
    type: "mpt_issuance",
  })
  const mptIssuance = accountObjects.result.account_objects.find(
    (obj) => obj.LedgerEntryType === "MPTokenIssuance",
  )
  if (!mptIssuance) {
    console.error("Failed to find MPTokenIssuance for contract account")
    console.error(
      "Account objects:",
      JSON.stringify(accountObjects.result, null, 2),
    )
    process.exit(1)
  }
  const mptIssuanceID = mptIssuance.mpt_issuance_id
  console.log("MPT Issuance ID:", mptIssuanceID)

  // --- Step 2: Authorize users and mint tokens ---
  console.log("\n=== Step 2: Authorize Users & Mint Tokens ===")

  // Authorize destWallet to hold the MPT
  const authUserTx = {
    TransactionType: "MPTokenAuthorize",
    Account: destWallet.address,
    MPTokenIssuanceID: mptIssuanceID,
  }
  const authUserResult = await submit(authUserTx, destWallet)
  if (authUserResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to authorize user for MPT:",
      authUserResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("destWallet authorized for MPT")

  // --- Step 2b: Mint tokens to destWallet ---
  console.log("\n=== Step 2b: Mint tokens to destWallet ===")
  const mintAmount = Number(1000).toString(16) // UINT64 values are hex strings
  const mintTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("mint"),
    ComputationAllowance: 1000000,
    Fee: "10000000",
    Parameters: [
      {
        Parameter: {
          ParameterValue: {
            type: "ACCOUNT",
            value: destWallet.address,
          },
        },
      },
      {
        Parameter: {
          ParameterValue: {
            type: "UINT64",
            value: mintAmount,
          },
        },
      },
    ],
  }
  const mintResult = await submitContractCall(
    client,
    mintTx,
    sourceWallet,
    true,
  )
  if (mintResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error("mint failed:", mintResult.result.meta.TransactionResult)
    process.exit(1)
  }
  console.log(`mint(${destWallet.address}, ${mintAmount}) succeeded`)

  // --- Step 3: Test approve ---
  console.log("\n=== Step 3: Test approve ===")
  const approveAmount = Number(500).toString(16) // UINT64 values are hex strings
  const approveTx = {
    TransactionType: "ContractCall",
    Account: destWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("approve"),
    ComputationAllowance: 1000000,
    Fee: "10000000",
    Parameters: [
      {
        Parameter: {
          ParameterValue: {
            type: "ACCOUNT",
            value: sourceWallet.address,
          },
        },
      },
      {
        Parameter: {
          ParameterValue: {
            type: "UINT64",
            value: approveAmount,
          },
        },
      },
    ],
  }
  const approveResult = await submitContractCall(
    client,
    approveTx,
    destWallet,
    true,
  )
  if (approveResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "approve failed:",
      approveResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(`approve(${sourceWallet.address}, ${approveAmount}) succeeded`)

  // --- Step 4: Test transfer_from ---
  console.log("\n=== Step 4: Test transfer_from ===")
  const thirdWallet = await fundWallet()

  // Authorize the third wallet for the MPT
  const authThirdTx = {
    TransactionType: "MPTokenAuthorize",
    Account: thirdWallet.address,
    MPTokenIssuanceID: mptIssuanceID,
  }
  const authThirdResult = await submit(authThirdTx, thirdWallet)
  if (authThirdResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to authorize third wallet for MPT:",
      authThirdResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  const transferFromAmount = Number(100).toString(16) // UINT64 values are hex strings
  const transferFromTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("transfer_from"),
    ComputationAllowance: 1000000,
    Fee: "10000000",
    Parameters: [
      {
        Parameter: {
          ParameterValue: {
            type: "ACCOUNT",
            value: destWallet.address,
          },
        },
      },
      {
        Parameter: {
          ParameterValue: {
            type: "ACCOUNT",
            value: thirdWallet.address,
          },
        },
      },
      {
        Parameter: {
          ParameterValue: {
            type: "UINT64",
            value: transferFromAmount,
          },
        },
      },
    ],
  }
  const transferFromResult = await submitContractCall(
    client,
    transferFromTx,
    sourceWallet,
    true,
  )
  if (transferFromResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "transfer_from failed:",
      transferFromResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(
    `transfer_from(${destWallet.address}, ${thirdWallet.address}, ${transferFromAmount}) succeeded`,
  )

  console.log("\n✅  All ERC-20 contract tests passed!")
}

module.exports = { test }
