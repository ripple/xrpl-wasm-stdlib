async function test(testContext) {
  const { deploy, submit, sourceWallet, finish } = testContext
  // This escrow should always succeed
  // If it fails, something in rippled is broken
  const { sequence } = await deploy(sourceWallet, sourceWallet, finish)
  const txFail = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: sequence,
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
