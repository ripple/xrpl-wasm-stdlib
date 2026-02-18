const xrpl = require("xrpl")

async function test(testContext) {
  const { client, finish, submit, sourceWallet, destWallet, fundWallet } =
    testContext

  // Create a custom escrow with comprehensive fields for testing
  await client.connect()
  console.log("connected")

  // close_time is in seconds since Ripple Epoch (Jan 1, 2000 00:00 UTC)
  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  // Create escrow with both Condition and FinishFunction
  // IMPORTANT: Condition must be in full crypto-condition format (39 bytes), not just the hash (32 bytes)
  // Format: A0258020<32-byte-hash>810100
  const condition =
    "A0258020121B69A8D20269CFA850F78931EFF3B1FCF3CCA1982A22D7FDB111734C65E5E3810103"
  const fulfillment = "A0058003736868"

  console.log("\n=== Condition/Fulfillment Verification ===")
  console.log("Condition (full crypto-condition):", condition)
  console.log("  Length:", condition.length / 2, "bytes")
  console.log("Fulfillment:", fulfillment)
  console.log("  Length:", fulfillment.length / 2, "bytes")

  const escrowCreateTx = {
    TransactionType: "EscrowCreate",
    Account: sourceWallet.address,
    Amount: "100000",
    Destination: destWallet.address,
    CancelAfter: close_time + 2000,
    SourceTag: 11747,
    DestinationTag: 23480,
    Condition: condition,
    FinishFunction: finish,
  }

  console.log("EscrowCreate transaction:", escrowCreateTx)

  const createResponse = await submit(escrowCreateTx, sourceWallet)
  if (createResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create escrow:",
      createResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  const offerSequence = createResponse.result.tx_json.Sequence
  console.log(
    `Created escrow with both Condition and FinishFunction at sequence ${offerSequence}`,
  )

  // Setup multi-signing for the EscrowFinish transaction
  // We need at least 2 signers for the test
  const signer1 = await fundWallet()
  const signer2 = await fundWallet()

  // Set up SignerList on sourceWallet
  const signerListTx = {
    TransactionType: "SignerListSet",
    Account: sourceWallet.address,
    SignerQuorum: 2,
    SignerEntries: [
      {
        SignerEntry: {
          Account: signer1.address,
          SignerWeight: 1,
        },
      },
      {
        SignerEntry: {
          Account: signer2.address,
          SignerWeight: 1,
        },
      },
    ],
  }

  const signerListResponse = await submit(signerListTx, sourceWallet)
  if (signerListResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to set signer list:",
      signerListResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Create actual credentials for testing
  // Credential 1
  const credentialCreate1 = {
    TransactionType: "CredentialCreate",
    Account: signer1.address,
    Subject: sourceWallet.address,
    CredentialType: Buffer.from("TestCredential1", "utf8").toString("hex"),
  }
  const cred1Response = await submit(credentialCreate1, signer1)
  if (cred1Response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create credential 1:",
      cred1Response.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Credential 2
  const credentialCreate2 = {
    TransactionType: "CredentialCreate",
    Account: signer2.address,
    Subject: sourceWallet.address,
    CredentialType: Buffer.from("TestCredential2", "utf8").toString("hex"),
  }
  const cred2Response = await submit(credentialCreate2, signer2)
  if (cred2Response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create credential 2:",
      cred2Response.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Extract credential IDs from the metadata
  // Look for CreatedNode with LedgerEntryType: "Credential"
  const credentialID1 = cred1Response.result.meta.AffectedNodes.find(
    (node) => node.CreatedNode?.LedgerEntryType === "Credential",
  )?.CreatedNode?.LedgerIndex

  const credentialID2 = cred2Response.result.meta.AffectedNodes.find(
    (node) => node.CreatedNode?.LedgerEntryType === "Credential",
  )?.CreatedNode?.LedgerIndex

  if (!credentialID1 || !credentialID2) {
    console.error("Failed to extract credential IDs from metadata")
    process.exit(1)
  }

  console.log("Created credential 1:", credentialID1)
  console.log("Created credential 2:", credentialID2)

  // Accept the credentials (subject must accept them before they can be used)
  const credentialAccept1 = {
    TransactionType: "CredentialAccept",
    Account: sourceWallet.address,
    Issuer: signer1.address,
    CredentialType: credentialCreate1.CredentialType,
  }
  const accept1Response = await submit(credentialAccept1, sourceWallet)
  if (accept1Response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to accept credential 1:",
      accept1Response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("Accepted credential 1")

  const credentialAccept2 = {
    TransactionType: "CredentialAccept",
    Account: sourceWallet.address,
    Issuer: signer2.address,
    CredentialType: credentialCreate2.CredentialType,
  }
  const accept2Response = await submit(credentialAccept2, sourceWallet)
  if (accept2Response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to accept credential 2:",
      accept2Response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("Accepted credential 2")

  // Now finish the escrow
  console.log("\n=== EscrowFinish Transaction ===")

  const escrowFinishTx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    Condition: condition,
    Fulfillment: fulfillment,
    ComputationAllowance: 1000000,
    SourceTag: 12345,
    // Add the created credentials
    CredentialIDs: [credentialID1, credentialID2],
    // Add a memo for testing
    Memos: [
      {
        Memo: {
          MemoType: Buffer.from("test/escrow-finish", "utf8").toString("hex"),
          MemoData: Buffer.from("Testing EscrowFinish fields", "utf8").toString(
            "hex",
          ),
          MemoFormat: Buffer.from("text/plain", "utf8").toString("hex"),
        },
      },
    ],
  }

  console.log("EscrowFinish transaction:", escrowFinishTx)

  // Prepare the transaction for multi-signing
  const prepared = await client.autofill(escrowFinishTx)

  // Sign with both signers
  const signed1 = signer1.sign(prepared, true)
  const signed2 = signer2.sign(prepared, true)

  // Combine the signatures
  const multisignedTx = xrpl.multisign([signed1.tx_blob, signed2.tx_blob])

  // Submit the multi-signed transaction
  const response = await client.submitAndWait(multisignedTx)
  console.log("SUBMITTED EscrowFinish (multi-signed)")
  console.log("Result code: " + response.result?.meta?.TransactionResult)

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  console.log("✅  Successfully finished escrow with FinishFunction")

  await client.disconnect()
}

module.exports = { test }
