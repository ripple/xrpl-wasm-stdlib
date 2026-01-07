async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // This is a bit of a dummy example and test
  // The Smart Escrow just checks whether the ledger sequence is greater than 5
  // which is essentially guaranteed to already be true, even when running on standalone mode

  const txFail = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  const responseFail = await submit(txFail, sourceWallet)

  if (responseFail.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      responseFail.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
