async function test(testContext) {
  const {
    client,
    finish,
    submit,
    destWallet,
    fundWallet,
    expectResult,
    getLedgerCloseTime,
    finishEscrow,
  } = testContext

  const testAccount = await fundWallet()
  console.log(`Test account created: ${testAccount.address}`)

  // Enable DefaultRipple on the issuer so an AMM can be created against its IOU.
  expectResult(
    await submit(
      {
        TransactionType: "AccountSet",
        Account: destWallet.address,
        SetFlag: 8,
      },
      destWallet,
    ),
    "tesSUCCESS",
    "AccountSet DefaultRipple",
  )

  // Trust line for a USD IOU.
  const currency = "USD"
  expectResult(
    await submit(
      {
        TransactionType: "TrustSet",
        Account: testAccount.address,
        LimitAmount: {
          currency,
          issuer: destWallet.address,
          value: "10000",
        },
        Flags: 0,
      },
      testAccount,
    ),
    "tesSUCCESS",
    "TrustSet",
  )

  // Issue some IOU to the test account.
  expectResult(
    await submit(
      {
        TransactionType: "Payment",
        Account: destWallet.address,
        Destination: testAccount.address,
        Amount: { currency, issuer: destWallet.address, value: "1000" },
      },
      destWallet,
    ),
    "tesSUCCESS",
    "Payment (IOU)",
  )

  // AMM (XRP + IOU) so AMMID is set on the account.
  const currentLedger = (
    await client.request({ command: "ledger", ledger_index: "validated" })
  ).result.ledger_index
  expectResult(
    await submit(
      {
        TransactionType: "AMMCreate",
        Account: testAccount.address,
        Amount: "1000000",
        Amount2: { currency, issuer: destWallet.address, value: "100" },
        TradingFee: 500,
        LastLedgerSequence: currentLedger + 5,
      },
      testAccount,
    ),
    "tesSUCCESS",
    "AMMCreate",
  )
  console.log("AMM created — AMMID should now be set on account")

  // AccountSet with all compatible optional fields in one shot.
  expectResult(
    await submit(
      {
        TransactionType: "AccountSet",
        Account: testAccount.address,
        SetFlag: 5,
        Domain: Buffer.from("example.com", "utf8").toString("hex"),
        EmailHash: "5D41402ABC4B2A76B9719D911017C592",
        MessageKey:
          "03AB40A0490F9B7ED8DF29D246BF2D6269820A0EE7742ACDD457BEA7C7D0931EDB",
        TransferRate: 1002000000,
        TickSize: 5,
        WalletLocator:
          "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
      },
      testAccount,
    ),
    "tesSUCCESS",
    "AccountSet (all optional fields)",
  )

  // RegularKey.
  const regularKeyWallet = await fundWallet()
  expectResult(
    await submit(
      {
        TransactionType: "SetRegularKey",
        Account: testAccount.address,
        RegularKey: regularKeyWallet.address,
      },
      testAccount,
    ),
    "tesSUCCESS",
    "SetRegularKey",
  )

  // NFToken mint → sets FirstNFTokenSequence + MintedNFTokens.
  expectResult(
    await submit(
      {
        TransactionType: "NFTokenMint",
        Account: testAccount.address,
        NFTokenTaxon: 0,
        Flags: 8,
      },
      testAccount,
    ),
    "tesSUCCESS",
    "NFTokenMint",
  )

  // NFTokenMinter authorization.
  const minterWallet = await fundWallet()
  expectResult(
    await submit(
      {
        TransactionType: "AccountSet",
        Account: testAccount.address,
        NFTokenMinter: minterWallet.address,
        SetFlag: 10,
      },
      testAccount,
    ),
    "tesSUCCESS",
    "AccountSet NFTokenMinter",
  )

  // TicketCreate to set TicketCount.
  expectResult(
    await submit(
      {
        TransactionType: "TicketCreate",
        Account: testAccount.address,
        TicketCount: 5,
      },
      testAccount,
    ),
    "tesSUCCESS",
    "TicketCreate",
  )

  console.log(`Test account fully configured: ${testAccount.address}`)

  // Deploy an escrow whose Bytecode walks the AccountRoot fields.
  const close_time = await getLedgerCloseTime(client)
  const createResponse = await submit(
    {
      TransactionType: "EscrowCreate",
      Account: testAccount.address,
      Amount: "100000",
      Destination: destWallet.address,
      CancelAfter: close_time + 2000,
      Bytecode: finish,
    },
    testAccount,
  )
  expectResult(createResponse, "tesSUCCESS", "EscrowCreate")
  const offerSequence = createResponse.result.tx_json.Sequence

  await finishEscrow(testContext, testAccount, {
    Owner: testAccount.address,
    OfferSequence: offerSequence,
  })
  console.log("✅  Successfully finished escrow with Bytecode")
}

module.exports = { test }
