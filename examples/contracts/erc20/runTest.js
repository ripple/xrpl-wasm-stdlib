const xrpl = require("@transia/xrpl")

/**
 * Integration test for the ERC-20 MPT wrapper contract.
 *
 * Test flow:
 * 1. Create an MPT issuance
 * 2. Deploy the ERC-20 contract with the MPT as an instance parameter
 * 3. Authorize and fund the contract with MPTs
 * 4. Test total_supply, balance_of, approve, allowance, transfer, transfer_from
 */
async function test(testContext) {
  const { client, submit, sourceWallet, destWallet, fundWallet, finish } =
    testContext

  // --- Step 1: Create an MPT Issuance ---
  console.log("\n=== Step 1: Create MPT Issuance ===")
  const issuer = sourceWallet

  const mptCreateTx = {
    TransactionType: "MPTokenIssuanceCreate",
    Account: issuer.address,
    MaximumAmount: "1000000",
    AssetScale: 0,
  }
  const mptCreateResult = await submit(mptCreateTx, issuer)
  if (mptCreateResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to create MPT issuance:",
      mptCreateResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Extract the MPTokenIssuanceID from the metadata
  let mptIssuanceID = mptCreateResult.result.meta.mpt_issuance_id

  // --- Step 2: Deploy the ERC-20 contract ---
  console.log("\n=== Step 2: Deploy ERC-20 Contract ===")

  // The instance parameter is an Amount::MPT value
  // Format: { mpt_issuance_id: <hex>, value: "0" }
  const contractCreateTx = {
    TransactionType: "ContractCreate",
    Account: issuer.address,
    ContractCode: finish,
    ComputationAllowance: 1000000,
    Functions: [
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("total_supply"),
          Parameters: [],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("balance_of"),
          Parameters: [{ Parameter: { ParameterType: "ACCOUNT" } }],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("transfer"),
          Parameters: [
            { Parameter: { ParameterType: "ACCOUNT" } },
            { Parameter: { ParameterType: "UINT64" } },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("approve"),
          Parameters: [
            { Parameter: { ParameterType: "ACCOUNT" } },
            { Parameter: { ParameterType: "UINT64" } },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("allowance"),
          Parameters: [
            { Parameter: { ParameterType: "ACCOUNT" } },
            { Parameter: { ParameterType: "ACCOUNT" } },
          ],
        },
      },
      {
        Function: {
          FunctionName: xrpl.convertStringToHex("transfer_from"),
          Parameters: [
            { Parameter: { ParameterType: "ACCOUNT" } },
            { Parameter: { ParameterType: "ACCOUNT" } },
            { Parameter: { ParameterType: "UINT64" } },
          ],
        },
      },
    ],
    InstanceParameters: [
      {
        InstanceParameter: {
          ParameterType: "AMOUNT",
        },
      },
    ],
    InstanceParameterValues: [
      {
        InstanceParameterValue: {
          ParameterValue: {
            type: "AMOUNT",
            value: {
              mpt_issuance_id: mptIssuanceID,
              value: "0",
            },
          },
        },
      },
    ],
  }

  const contractCreateResult = await submit(contractCreateTx, issuer, true)
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
  console.log("Contract account:", contractAccount)

  // --- Step 3: Authorize the contract to hold the MPT and mint tokens ---
  console.log("\n=== Step 3: Authorize & Fund Contract ===")

  // Authorize the destWallet (a regular user) to hold the MPT
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
  console.log("User authorized for MPT")

  // Mint tokens to the issuer (issuer can hold their own MPTs)
  // and then send some to the contract account
  const mintAmount = "10000"
  const mintTx = {
    TransactionType: "Payment",
    Account: issuer.address,
    Destination: destWallet.address,
    Amount: {
      mpt_issuance_id: mptIssuanceID,
      value: mintAmount,
    },
  }
  const mintResult = await submit(mintTx, issuer)
  if (mintResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Failed to mint MPTs:",
      mintResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(`Minted ${mintAmount} MPTs to ${destWallet.address}`)

  // --- Step 4: Test total_supply ---
  console.log("\n=== Step 4: Test total_supply ===")
  const totalSupplyTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("total_supply"),
    ComputationAllowance: 1000000,
  }
  const totalSupplyResult = await submit(totalSupplyTx, sourceWallet, true)
  if (totalSupplyResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "total_supply failed:",
      totalSupplyResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("total_supply call succeeded")

  // --- Step 5: Test balance_of ---
  console.log("\n=== Step 5: Test balance_of ===")
  const balanceOfTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("balance_of"),
    ComputationAllowance: 1000000,
    Parameters: [
      {
        Parameter: {
          ParameterValue: {
            type: "ACCOUNT",
            value: destWallet.address,
          },
        },
      },
    ],
  }
  const balanceOfResult = await submit(balanceOfTx, sourceWallet, true)
  if (balanceOfResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "balance_of failed:",
      balanceOfResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("balance_of call succeeded")

  // --- Step 6: Test approve ---
  console.log("\n=== Step 6: Test approve ===")
  const approveAmount = "500"
  const approveTx = {
    TransactionType: "ContractCall",
    Account: destWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("approve"),
    ComputationAllowance: 1000000,
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
  const approveResult = await submit(approveTx, destWallet, true)
  if (approveResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "approve failed:",
      approveResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log(`approve(${sourceWallet.address}, ${approveAmount}) succeeded`)

  // --- Step 7: Test allowance ---
  console.log("\n=== Step 7: Test allowance ===")
  const allowanceTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("allowance"),
    ComputationAllowance: 1000000,
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
            value: sourceWallet.address,
          },
        },
      },
    ],
  }
  const allowanceResult = await submit(allowanceTx, sourceWallet, true)
  if (allowanceResult.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "allowance failed:",
      allowanceResult.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("allowance call succeeded")

  // --- Step 8: Test transfer_from ---
  console.log("\n=== Step 8: Test transfer_from ===")
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

  const transferFromAmount = "100"
  const transferFromTx = {
    TransactionType: "ContractCall",
    Account: sourceWallet.address,
    ContractAccount: contractAccount,
    FunctionName: xrpl.convertStringToHex("transfer_from"),
    ComputationAllowance: 1000000,
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
  const transferFromResult = await submit(transferFromTx, sourceWallet, true)
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
