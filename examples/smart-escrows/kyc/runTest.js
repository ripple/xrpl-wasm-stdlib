const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  const txFail = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  // Submitting EscrowFinish transaction...
  // This should fail since the credential hasn't been created yet
  const responseFail = await submit(txFail, sourceWallet)

  if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.log("\nEscrow finished successfully?????")
    process.exit(1)
  }

  const credTx = {
    TransactionType: "CredentialCreate",
    Account: destWallet.address,
    Subject: destWallet.address,
    CredentialType: xrpl.convertStringToHex("termsandconditions"),
    URI: xrpl.convertStringToHex("https://example.com/terms"),
  }

  // Submitting CredentialCreate transaction...
  const credResponse = await submit(credTx, destWallet)

  if (credResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create credential:",
      credResponse.result.meta.TransactionResult,
    )
  }

  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  // Submitting EscrowFinish transaction...
  const response = await submit(tx, sourceWallet)

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
