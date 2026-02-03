const xrpl = require("xrpl")
const path = require("path")
const fs = require("fs")

async function test(testContext) {
  const { deploy, finish, submit, sourceWallet, destWallet } = testContext

  // Complete atomic swap test suite
  // Tests atomic_swap1 (Alice→Bob) and atomic_swap2 (Bob→Alice) with two-phase execution
  // Validates both escrows complete successfully

  // Load atomic_swap2 WASM for the counterpart escrow
  const atomicSwap2Path = path.resolve(
    __dirname,
    "../../target/wasm32v1-none/release/atomic_swap2.wasm",
  )
  let atomicSwap2Wasm
  try {
    const wasmData = fs.readFileSync(atomicSwap2Path)
    atomicSwap2Wasm = wasmData.toString("hex")
  } catch (err) {
    console.error(
      `Failed to load atomic_swap2 WASM from ${atomicSwap2Path}:`,
      err.message,
    )
    console.error("Make sure atomic_swap2 is built first!")
    process.exit(1)
  }

  // Deploy atomic_swap1 escrow (Alice → Bob)
  const swap1Result = await deploy(sourceWallet, destWallet, finish)

  // Deploy atomic_swap2 escrow (Bob → Alice) with atomic_swap1's keylet in data field
  const swap2Result = await deploy(
    destWallet,
    sourceWallet,
    atomicSwap2Wasm,
    swap1Result.escrowKeylet,
  )

  // Phase 1: Execute atomic_swap2 Phase 1 - Initialize timing
  // Phase 1 should return tecWASM_REJECTED (contract returns 0) but data update persists
  const txSwap2Phase1 = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(swap2Result.sequence),
    ComputationAllowance: 1000000,
  }

  const responseSwap2Phase1 = await submit(txSwap2Phase1, destWallet)
  if (
    responseSwap2Phase1.result.meta.TransactionResult !== "tecWASM_REJECTED"
  ) {
    console.error(
      "atomic_swap2 Phase 1 expected tecWASM_REJECTED, got:",
      responseSwap2Phase1.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Verify escrow still exists (Phase 1 should not consume it)
  const swap2StillExists = !responseSwap2Phase1.result.meta.AffectedNodes.some(
    (node) => node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
  )
  if (!swap2StillExists) {
    console.error("ERROR: atomic_swap2 Phase 1 incorrectly consumed the escrow")
    process.exit(1)
  }

  // Phase 2: Execute atomic_swap2 Phase 2 - Complete with timing validation
  const txSwap2Phase2 = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(swap2Result.sequence),
    ComputationAllowance: 1000000,
  }

  const responseSwap2Phase2 = await submit(txSwap2Phase2, destWallet)
  if (responseSwap2Phase2.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "atomic_swap2 Phase 2 failed:",
      responseSwap2Phase2.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Verify escrow was consumed
  const swap2Consumed = responseSwap2Phase2.result.meta.AffectedNodes.some(
    (node) => node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
  )
  if (!swap2Consumed) {
    console.error("ERROR: atomic_swap2 Phase 2 should have consumed the escrow")
    process.exit(1)
  }

  // Attempt to execute atomic_swap1 after counterpart consumed
  // This should fail because atomic_swap2 was already consumed in Phase 2
  // This demonstrates the atomic nature - once one escrow completes, the counterpart cannot validate it
  const txSwap1 = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(swap1Result.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: swap2Result.escrowKeylet,
        },
      },
    ],
  }

  const responseSwap1 = await submit(txSwap1, sourceWallet)
  if (responseSwap1.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected atomic_swap1 to fail after counterpart consumed, but got:",
      responseSwap1.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Test correct order: atomic_swap1 first, then atomic_swap2
  // Deploy fresh atomic_swap1 escrow (Alice → Bob)
  const finalSwap1Result = await deploy(sourceWallet, destWallet, finish)

  // Deploy fresh atomic_swap2 escrow (Bob → Alice)
  const finalSwap2Result = await deploy(
    destWallet,
    sourceWallet,
    atomicSwap2Wasm,
    finalSwap1Result.escrowKeylet,
  )

  // Execute atomic_swap2 Phase 1
  const txFinalSwap2Phase1 = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(finalSwap2Result.sequence),
    ComputationAllowance: 1000000,
  }

  const responseFinalSwap2Phase1 = await submit(txFinalSwap2Phase1, destWallet)
  // Phase 1 should return tecWASM_REJECTED (contract returns 0) but data update persists
  if (
    responseFinalSwap2Phase1.result.meta.TransactionResult !==
    "tecWASM_REJECTED"
  ) {
    console.error(
      "Final atomic_swap2 Phase 1 expected tecWASM_REJECTED, got:",
      responseFinalSwap2Phase1.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Execute atomic_swap1 BEFORE Phase 2 (while counterpart still exists)
  const txFinalSwap1 = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(finalSwap1Result.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: finalSwap2Result.escrowKeylet,
        },
      },
    ],
  }

  const responseFinalSwap1 = await submit(txFinalSwap1, sourceWallet)
  if (responseFinalSwap1.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Final atomic_swap1 failed:",
      responseFinalSwap1.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Verify atomic_swap1 escrow was consumed
  const finalSwap1Consumed = responseFinalSwap1.result.meta.AffectedNodes.some(
    (node) => node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
  )
  if (!finalSwap1Consumed) {
    console.error("ERROR: atomic_swap1 should have consumed the escrow")
    process.exit(1)
  }

  // Execute atomic_swap2 Phase 2
  const txFinalSwap2Phase2 = {
    TransactionType: "EscrowFinish",
    Account: destWallet.address,
    Owner: destWallet.address,
    OfferSequence: parseInt(finalSwap2Result.sequence),
    ComputationAllowance: 1000000,
  }

  const responseFinalSwap2Phase2 = await submit(txFinalSwap2Phase2, destWallet)
  if (responseFinalSwap2Phase2.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Final atomic_swap2 Phase 2 failed:",
      responseFinalSwap2Phase2.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Verify atomic_swap2 escrow was consumed
  const finalSwap2Consumed =
    responseFinalSwap2Phase2.result.meta.AffectedNodes.some(
      (node) =>
        node.DeletedNode && node.DeletedNode.LedgerEntryType === "Escrow",
    )
  if (!finalSwap2Consumed) {
    console.error("ERROR: atomic_swap2 Phase 2 should have consumed the escrow")
    process.exit(1)
  }
}

module.exports = { test }
