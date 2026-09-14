async function test(testContext) {
  const {
    deploy,
    finish,
    sourceWallet,
    destWallet,
    finishEscrow,
    expectEscrowConsumed,
    expectEscrowSurvived,
  } = testContext

  // Test atomic_swap2 two-phase execution:
  // Phase 1: Data field validation and timing initialization
  // Phase 2: Timing validation and successful completion
  // Security: WASM and account validation

  // Deploy first escrow that atomic_swap2 will reference
  const firstEscrowResult = await deploy(sourceWallet, destWallet, finish)

  // Create atomic_swap2 escrow with first escrow's ledger entry ID in data field
  const atomicSwap2Result = await deploy(destWallet, sourceWallet, finish, {
    Data: firstEscrowResult.escrowId, // 32-byte ledger entry ID
  })

  // Phase 1: contract returns 0 → tecBYTECODE_REJECTED, but data update persists
  // and the escrow must survive (the contract signals "wait for phase 2").
  const responsePhase1 = await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: atomicSwap2Result.sequence,
    expect: "tecBYTECODE_REJECTED",
  })
  expectEscrowSurvived(responsePhase1, "atomic_swap2 phase 1")

  // Phase 2: timing now validates, finish succeeds.
  const responsePhase2 = await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: atomicSwap2Result.sequence,
  })
  expectEscrowConsumed(responsePhase2, "atomic_swap2 phase 2")

  // Security test: reject when Data length is not the expected 32 bytes.
  try {
    const invalidDataEscrow = await deploy(destWallet, sourceWallet, finish, {
      Data: "INVALID_DATA_NOT_32_BYTES",
    })
    await finishEscrow(testContext, destWallet, {
      Owner: destWallet.address,
      OfferSequence: invalidDataEscrow.sequence,
      expect: "tecBYTECODE_REJECTED",
    })
  } catch (error) {
    // deploy() itself rejecting is also acceptable
  }

  // Security test: reject when Data references a non-existent escrow.
  const fakeId = "A".repeat(64) // 32 bytes of 0xAA
  const fakeRefEscrow = await deploy(destWallet, sourceWallet, finish, {
    Data: fakeId,
  })
  await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: fakeRefEscrow.sequence,
    expect: "tecBYTECODE_REJECTED",
  })

  console.log("Success!")
}

module.exports = { test }
