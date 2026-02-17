async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  // Test atomic_swap2 two-phase execution:
  // Phase 1: Data field validation and timing initialization
  // Phase 2: Timing validation and successful completion
  // Security: WASM and account validation

  // Deploy first escrow that atomic_swap2 will reference
  const firstEscrowResult = await deploy(sourceWallet, destWallet, finish)

  // Create atomic_swap2 escrow with first escrow's keylet in data field
  const atomicSwap2Result = await deploy(
    destWallet,
    sourceWallet,
    finish,
    firstEscrowResult.escrowKeylet, // 32-byte keylet in data field
  )

  // Phase 1: First finish attempt should initialize timing and return tecWASM_REJECTED
  // The contract returns 0 to indicate "wait for phase 2", but data update persists
  const txPhase1 = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(atomicSwap2Result.sequence),
    ComputationAllowance: 1000000,
  }

  const responsePhase1 = await submit(txPhase1, destWallet)

  if (responsePhase1.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "\nPhase 1 expected tecWASM_REJECTED, got:",
      responsePhase1.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Verify the escrow still exists (wasn't finished)
  const escrowStillExists = !responsePhase1.result.meta.AffectedNodes.some(
    (node) => node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
  )
  if (!escrowStillExists) {
    console.error(
      "\nPhase 1 incorrectly finished the escrow - it should still exist",
    )
    process.exit(1)
  }

  // Phase 2: Second finish attempt should validate timing and succeed
  const txPhase2 = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(atomicSwap2Result.sequence),
    ComputationAllowance: 1000000,
  }

  const responsePhase2 = await submit(txPhase2, destWallet)

  // Phase 2 should succeed and finish the escrow (since we're within deadline)
  if (responsePhase2.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nPhase 2 failed unexpectedly:",
      responsePhase2.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Verify the escrow was finished
  const escrowConsumed = responsePhase2.result.meta.AffectedNodes.some(
    (node) => node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
  )
  if (!escrowConsumed) {
    console.error("\nPhase 2 should have finished the escrow but didn't")
    process.exit(1)
  }

  // Security test: Try to create atomic_swap2 with invalid data
  try {
    const invalidDataEscrow = await deploy(
      destWallet,
      sourceWallet,
      finish,
      "INVALID_DATA_NOT_32_BYTES", // Wrong size data
    )

    const txInvalidData = {
      TransactionType: "EscrowFinish",
      Account: destWallet.address,
      Owner: destWallet.address,
      OfferSequence: parseInt(invalidDataEscrow.sequence),
      ComputationAllowance: 1000000,
    }

    const responseInvalidData = await submit(txInvalidData, destWallet)

    // Should fail due to invalid data field length
    if (
      responseInvalidData.result.meta.TransactionResult !== "tecWASM_REJECTED"
    ) {
      console.error(
        "\nSecurity test failed: escrow with invalid data should have been rejected:",
        responseInvalidData.result.meta.TransactionResult,
      )
      process.exit(1)
    }
  } catch (error) {
    // If deploy itself fails, that's also acceptable
  }

  // Security test: Try to reference non-existent escrow
  const fakeKeylet = "A".repeat(64) // 32 bytes of 0xAA
  const fakeRefEscrow = await deploy(
    destWallet,
    sourceWallet,
    finish,
    fakeKeylet,
  )

  const txFakeRef = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(fakeRefEscrow.sequence),
    ComputationAllowance: 1000000,
  }

  const responseFakeRef = await submit(txFakeRef, destWallet)

  // Should fail due to non-existent referenced escrow
  if (responseFakeRef.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "\nSecurity test failed: escrow with fake reference should have been rejected:",
      responseFakeRef.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  console.log("Success!")
}

module.exports = { test }
