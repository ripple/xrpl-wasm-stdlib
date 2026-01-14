const xrpl = require("xrpl")

async function test(testContext) {
  const { client, finish, submit, sourceWallet, destWallet, fundWallet } =
    testContext

  await client.connect()
  console.log("connected")

  // // Create a new wallet that we'll configure with all optional AccountRoot fields
  const testAccount = await fundWallet()
  console.log(`Test account created: ${testAccount.address}`)
  //
  // Create an AMM to set the AMMID field on the account
  // First, enable DefaultRipple on the issuer account
  const issuerAccountSetTx = {
    TransactionType: "AccountSet",
    Account: destWallet.address,
    SetFlag: 8, // asfDefaultRipple - allows rippling by default
  }
  const issuerAccountSetResponse = await submit(issuerAccountSetTx, destWallet)
  if (issuerAccountSetResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to set DefaultRipple on issuer:",
      issuerAccountSetResponse.result.meta.TransactionResult,
    )
    console.error(
      "Full response:",
      JSON.stringify(issuerAccountSetResponse, null, 2),
    )
    process.exit(1)
  }
  console.log("Issuer DefaultRipple enabled")

  // Now create a trust line for an IOU currency
  const currency = "USD"
  const trustSetTx = {
    TransactionType: "TrustSet",
    Account: testAccount.address,
    LimitAmount: {
      currency: currency,
      issuer: destWallet.address,
      value: "10000",
    },
    Flags: 0, // Enable rippling (no tfSetNoRipple flag)
  }
  const trustSetResponse = await submit(trustSetTx, testAccount)
  if (trustSetResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create trust line:",
      trustSetResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(trustSetResponse, null, 2))
    process.exit(1)
  }
  console.log("Trust line created")

  // Issue some IOU to the test account
  const paymentTx = {
    TransactionType: "Payment",
    Account: destWallet.address,
    Destination: testAccount.address,
    Amount: {
      currency: currency,
      issuer: destWallet.address,
      value: "1000",
    },
  }
  const paymentResponse = await submit(paymentTx, destWallet)
  if (paymentResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to issue IOU:",
      paymentResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(paymentResponse, null, 2))
    process.exit(1)
  }
  console.log("IOU issued")

  // Create an AMM with XRP and the IOU
  // AMM creation is complex and may take longer, so we manually set a higher LastLedgerSequence
  ledgerInfo = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  currentLedger = ledgerInfo.result.ledger_index

  const ammCreateTx = {
    TransactionType: "AMMCreate",
    Account: testAccount.address,
    Amount: "1000000", // 1 XRP in drops
    Amount2: {
      currency: currency,
      issuer: destWallet.address,
      value: "100",
    },
    TradingFee: 500, // 0.5% trading fee (in basis points, max 1000)
    LastLedgerSequence: currentLedger + 5,
  }
  const ammCreateResponse = await submit(ammCreateTx, testAccount)
  if (ammCreateResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create AMM:",
      ammCreateResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(ammCreateResponse, null, 2))
    process.exit(1)
  }

  console.log("AMM created successfully - AMMID should now be set on account")

  // Set all compatible AccountSet fields in a single transaction
  const accountSetTx = {
    TransactionType: "AccountSet",
    Account: testAccount.address,
    SetFlag: 5, // asfAccountTxnID - enables AccountTxnID tracking
    Domain: Buffer.from("example.com", "utf8").toString("hex"),
    EmailHash: "5D41402ABC4B2A76B9719D911017C592", // MD5 of "hello"
    MessageKey:
      "03AB40A0490F9B7ED8DF29D246BF2D6269820A0EE7742ACDD457BEA7C7D0931EDB",
    TransferRate: 1002000000, // 0.2% transfer fee
    TickSize: 5, // Must be 3-15
    WalletLocator:
      "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    // Note: WalletSize is unused and cannot be set via AccountSet
  }
  const accountSetResponse = await submit(accountSetTx, testAccount)
  if (accountSetResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to configure account:",
      accountSetResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(accountSetResponse, null, 2))
    process.exit(1)
  }
  console.log("Account configured")

  // Set a RegularKey on the account
  const regularKeyWallet = await fundWallet()
  console.log(`Regular key wallet created: ${regularKeyWallet.address}`)

  const setRegularKeyTx = {
    TransactionType: "SetRegularKey",
    Account: testAccount.address,
    RegularKey: regularKeyWallet.address,
  }
  const setRegularKeyResponse = await submit(setRegularKeyTx, testAccount)
  if (setRegularKeyResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to set regular key:",
      setRegularKeyResponse.result.meta.TransactionResult,
    )
    console.error(
      "Full response:",
      JSON.stringify(setRegularKeyResponse, null, 2),
    )
    process.exit(1)
  }
  console.log("RegularKey set")

  // Mint an NFToken to set FirstNFTokenSequence and MintedNFTokens
  const nftMintTx = {
    TransactionType: "NFTokenMint",
    Account: testAccount.address,
    NFTokenTaxon: 0,
    Flags: 8, // tfTransferable
  }
  const nftMintResponse = await submit(nftMintTx, testAccount)
  if (nftMintResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to mint NFToken:",
      nftMintResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(nftMintResponse, null, 2))
    process.exit(1)
  }
  console.log(
    "NFToken minted - FirstNFTokenSequence and MintedNFTokens should now be set",
  )

  // Set NFTokenMinter by enabling the asfAuthorizedNFTokenMinter flag
  // First, create a minter account
  const minterWallet = await fundWallet()
  console.log(`Minter wallet created: ${minterWallet.address}`)

  const setNFTokenMinterTx = {
    TransactionType: "AccountSet",
    Account: testAccount.address,
    NFTokenMinter: minterWallet.address,
    SetFlag: 10, // asfAuthorizedNFTokenMinter
  }
  const setNFTokenMinterResponse = await submit(setNFTokenMinterTx, testAccount)
  if (setNFTokenMinterResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to set NFTokenMinter:",
      setNFTokenMinterResponse.result.meta.TransactionResult,
    )
    console.error(
      "Full response:",
      JSON.stringify(setNFTokenMinterResponse, null, 2),
    )
    process.exit(1)
  }
  console.log("NFTokenMinter set")

  // Create a ticket to set TicketCount
  const ticketCreateTx = {
    TransactionType: "TicketCreate",
    Account: testAccount.address,
    TicketCount: 5,
  }
  const ticketResponse = await submit(ticketCreateTx, testAccount)
  if (ticketResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create tickets:",
      ticketResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(ticketResponse, null, 2))
    process.exit(1)
  }
  console.log("Tickets created")

  console.log(
    `Test account configured with all optional fields: ${testAccount.address}`,
  )

  // Now create an escrow using the test account and deploy the WASM
  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  const escrowCreateTx = {
    TransactionType: "EscrowCreate",
    Account: testAccount.address,
    Amount: "100000",
    Destination: destWallet.address,
    CancelAfter: close_time + 2000,
    FinishFunction: finish,
  }

  const createResponse = await submit(escrowCreateTx, testAccount)
  if (createResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create escrow:",
      createResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(createResponse, null, 2))
    process.exit(1)
  }
  console.log("Escrow created")

  const offerSequence = createResponse.result.tx_json.Sequence

  // Finish the escrow to trigger WASM execution
  const tx = {
    TransactionType: "EscrowFinish",
    Account: testAccount.address,
    Owner: testAccount.address,
    OfferSequence: parseInt(offerSequence),
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, testAccount)

  await client.disconnect()

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(response, null, 2))
    process.exit(1)
  }

  console.log("✅  Successfully finished escrow with FinishFunction")
}

module.exports = { test }
