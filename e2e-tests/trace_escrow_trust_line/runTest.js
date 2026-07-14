const xrpl = require("xrpl")

async function test(testContext) {
  const { client, finish, submit, destWallet, fundWallet } = testContext

  await client.connect()
  console.log("connected")

  // The holder account: finishes the escrow and holds the IOU trust line.
  const holder = await fundWallet()
  console.log(`Holder account created: ${holder.address}`)

  // The issuer of the IOU is the escrow destination (destWallet). Enable DefaultRipple so the
  // trust line balance ripples normally.
  const issuerAccountSetTx = {
    TransactionType: "AccountSet",
    Account: destWallet.address,
    SetFlag: 8, // asfDefaultRipple
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

  // Create the trust line (RippleState object) between holder and issuer for USD.
  const currency = "USD"
  const trustSetTx = {
    TransactionType: "TrustSet",
    Account: holder.address,
    LimitAmount: {
      currency: currency,
      issuer: destWallet.address,
      value: "10000",
    },
    Flags: 0, // Enable rippling (no tfSetNoRipple flag)
  }
  const trustSetResponse = await submit(trustSetTx, holder)
  if (trustSetResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create trust line:",
      trustSetResponse.result.meta.TransactionResult,
    )
    console.error("Full response:", JSON.stringify(trustSetResponse, null, 2))
    process.exit(1)
  }
  console.log("Trust line created")

  // Issue some IOU to the holder so the trust line carries a non-zero IOU balance.
  const paymentTx = {
    TransactionType: "Payment",
    Account: destWallet.address,
    Destination: holder.address,
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

  // Create an escrow finished by the holder, with the issuer as destination, deploying the WASM.
  // The contract reads the finishing account (holder) and the escrow destination (issuer) to
  // reconstruct the trust line's two parties.
  const close_time = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time

  const escrowCreateTx = {
    TransactionType: "EscrowCreate",
    Account: holder.address,
    Amount: "100000",
    Destination: destWallet.address,
    CancelAfter: close_time + 2000,
    FinishFunction: finish,
  }

  const createResponse = await submit(escrowCreateTx, holder)
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

  // Finish the escrow to trigger WASM execution.
  const tx = {
    TransactionType: "EscrowFinish",
    Account: holder.address,
    Owner: holder.address,
    OfferSequence: parseInt(offerSequence),
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, holder)

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
