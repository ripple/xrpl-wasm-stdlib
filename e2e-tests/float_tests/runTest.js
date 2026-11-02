async function test(testContext) {
  const { deploy, sourceWallet, finish, finishEscrow } = testContext
  // This escrow should always succeed. If it fails, something in rippled
  // is broken.
  const { sequence } = await deploy(sourceWallet, sourceWallet, finish)
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: sequence,
  })
}

module.exports = { test }
