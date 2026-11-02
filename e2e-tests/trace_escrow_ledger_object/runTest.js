async function test(testContext) {
  const { client, finish, submit, sourceWallet, destWallet, expectResult } =
    testContext

  // Condition must be in full crypto-condition format (39 bytes), not just
  // the hash: A0258020<32-byte-hash>810100
  const condition =
    "A0258020121B69A8D20269CFA850F78931EFF3B1FCF3CCA1982A22D7FDB111734C65E5E3810103"
  const fulfillment = "A0058003736868"

  const close_time = (
    await client.request({ command: "ledger", ledger_index: "closed" })
  ).result.ledger.close_time
  const finishAfter = close_time + (process.env.DEVNET ? 3 : 0)

  const createResponse = await submit(
    {
      TransactionType: "EscrowCreate",
      Account: sourceWallet.address,
      Amount: "1000000",
      Destination: destWallet.address,
      CancelAfter: close_time + 2000,
      FinishAfter: finishAfter,
      SourceTag: 11747,
      DestinationTag: 23480,
      Condition: condition,
      Bytecode: finish,
    },
    sourceWallet,
  )
  expectResult(createResponse, "tesSUCCESS", "EscrowCreate")
  const offerSequence = createResponse.result.tx_json.Sequence

  const response = await submit(
    {
      TransactionType: "EscrowFinish",
      Account: sourceWallet.address,
      Owner: sourceWallet.address,
      OfferSequence: parseInt(offerSequence),
      Condition: condition,
      Fulfillment: fulfillment,
      Gas: 1000000,
    },
    sourceWallet,
  )
  expectResult(response, "tesSUCCESS", "EscrowFinish (Condition + Bytecode)")
  console.log("✅  Successfully finished escrow with Bytecode")
}

module.exports = { test }
