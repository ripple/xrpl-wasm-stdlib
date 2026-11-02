async function test(testContext) {
  const { deploy, finish, sourceWallet, destWallet, finishEscrow } = testContext

  // The contract just checks that the ledger sequence is greater than 5
  // (essentially always true, even in standalone mode), so the finish
  // always succeeds.
  const escrowResult = await deploy(sourceWallet, destWallet, finish)
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
  })
}

module.exports = { test }
