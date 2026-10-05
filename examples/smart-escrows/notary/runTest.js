async function test(testContext) {
  const {
    sourceWallet,
    deploy,
    finish,
    destWallet,
    finishEscrow,
    getTestAccount,
  } = testContext

  // NOTARY_ACCOUNT in src/lib.rs.
  const notary = await getTestAccount(testContext, sourceWallet)

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // Non-notary submitter is rejected.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
    expect: "tecBYTECODE_REJECTED",
  })

  // Notary succeeds.
  await finishEscrow(testContext, notary, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
  })
}

module.exports = { test }
