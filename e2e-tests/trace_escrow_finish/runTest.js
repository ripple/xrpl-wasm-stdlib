const xrpl = require("xrpl")

async function test(testContext) {
  const {
    client,
    finish,
    submit,
    sourceWallet,
    destWallet,
    fundWallet,
    expectResult,
    getLedgerCloseTime,
  } = testContext

  // Condition must be in full crypto-condition format (39 bytes), not just
  // the hash: A0258020<32-byte-hash>810100
  const condition =
    "A0258020121B69A8D20269CFA850F78931EFF3B1FCF3CCA1982A22D7FDB111734C65E5E3810103"
  const fulfillment = "A0058003736868"

  const close_time = await getLedgerCloseTime(client)

  const createResponse = await submit(
    {
      TransactionType: "EscrowCreate",
      Account: sourceWallet.address,
      Amount: "100000",
      Destination: destWallet.address,
      CancelAfter: close_time + 2000,
      SourceTag: 11747,
      DestinationTag: 23480,
      Condition: condition,
      Bytecode: finish,
    },
    sourceWallet,
  )
  expectResult(createResponse, "tesSUCCESS", "EscrowCreate")
  const offerSequence = createResponse.result.tx_json.Sequence
  console.log(
    `Created escrow with both Condition and Bytecode at sequence ${offerSequence}`,
  )

  // Multi-signing setup: two signers with weight 1 each, quorum 2.
  const signer1 = await fundWallet()
  const signer2 = await fundWallet()

  expectResult(
    await submit(
      {
        TransactionType: "SignerListSet",
        Account: sourceWallet.address,
        SignerQuorum: 2,
        SignerEntries: [
          { SignerEntry: { Account: signer1.address, SignerWeight: 1 } },
          { SignerEntry: { Account: signer2.address, SignerWeight: 1 } },
        ],
      },
      sourceWallet,
    ),
    "tesSUCCESS",
    "SignerListSet",
  )

  // Two credentials, issued by each signer, subject = sourceWallet.
  const credType1 = Buffer.from("TestCredential1", "utf8").toString("hex")
  const credType2 = Buffer.from("TestCredential2", "utf8").toString("hex")

  const cred1Response = await submit(
    {
      TransactionType: "CredentialCreate",
      Account: signer1.address,
      Subject: sourceWallet.address,
      CredentialType: credType1,
    },
    signer1,
  )
  expectResult(cred1Response, "tesSUCCESS", "CredentialCreate #1")

  const cred2Response = await submit(
    {
      TransactionType: "CredentialCreate",
      Account: signer2.address,
      Subject: sourceWallet.address,
      CredentialType: credType2,
    },
    signer2,
  )
  expectResult(cred2Response, "tesSUCCESS", "CredentialCreate #2")

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
  console.log("Created credentials:", credentialID1, credentialID2)

  // Subject accepts both credentials.
  expectResult(
    await submit(
      {
        TransactionType: "CredentialAccept",
        Account: sourceWallet.address,
        Issuer: signer1.address,
        CredentialType: credType1,
      },
      sourceWallet,
    ),
    "tesSUCCESS",
    "CredentialAccept #1",
  )
  expectResult(
    await submit(
      {
        TransactionType: "CredentialAccept",
        Account: sourceWallet.address,
        Issuer: signer2.address,
        CredentialType: credType2,
      },
      sourceWallet,
    ),
    "tesSUCCESS",
    "CredentialAccept #2",
  )

  // EscrowFinish, multi-signed. We keep the raw xrpl.multisign path here
  // because it drops below the harness's finishEscrow surface — the harness
  // covers the common "one signer, one submit" case.
  const escrowFinishTx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(offerSequence),
    Condition: condition,
    Fulfillment: fulfillment,
    Gas: 1000000,
    SourceTag: 12345,
    CredentialIDs: [credentialID1, credentialID2],
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

  const prepared = await client.autofill(escrowFinishTx)
  const signed1 = signer1.sign(prepared, true)
  const signed2 = signer2.sign(prepared, true)
  const multisignedTx = xrpl.multisign([signed1.tx_blob, signed2.tx_blob])

  const response = await client.submitAndWait(multisignedTx)
  console.log("SUBMITTED EscrowFinish (multi-signed)")
  expectResult(response, "tesSUCCESS", "EscrowFinish (multi-signed)")
  console.log("✅  Successfully finished escrow with Bytecode")
}

module.exports = { test }
