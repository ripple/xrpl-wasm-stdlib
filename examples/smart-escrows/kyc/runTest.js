const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet, finishEscrow } =
    testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // Without the credential, finish must fail.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
    expect: "tecBYTECODE_REJECTED",
  })

  const credTx = {
    TransactionType: "CredentialCreate",
    Account: destWallet.address,
    Subject: destWallet.address,
    CredentialType: xrpl.convertStringToHex("termsandconditions"),
    URI: xrpl.convertStringToHex("https://example.com/terms"),
  }

  const credResponse = await submit(credTx, destWallet)
  if (credResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create credential:",
      credResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Once the credential exists, finish succeeds.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
  })
}

module.exports = { test }
