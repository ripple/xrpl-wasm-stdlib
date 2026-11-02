const xrpl = require("xrpl")

async function test(testContext) {
  const {
    deploy,
    finish,
    sourceWallet,
    destWallet,
    finishEscrow,
    expectEscrowConsumed,
    expectEscrowSurvived,
    loadWasmHex,
  } = testContext

  // Complete atomic swap test suite
  // Tests atomic_swap1 (Alice→Bob) and atomic_swap2 (Bob→Alice) with two-phase execution
  // Validates both escrows complete successfully

  const atomicSwap2Wasm = loadWasmHex(
    __dirname,
    "../../../target/wasm32v1-none/release/atomic_swap2.wasm",
  )

  // ── Attempt 1: Alice tries to finish AFTER Bob's counterpart has already ────
  //     been consumed. Alice must be rejected.

  const swap1Result = await deploy(sourceWallet, destWallet, finish)
  const swap2Result = await deploy(destWallet, sourceWallet, atomicSwap2Wasm, {
    Data: swap1Result.escrowId,
  })

  // atomic_swap2 phase 1 (tecBYTECODE_REJECTED, escrow survives).
  const responseSwap2Phase1 = await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: swap2Result.sequence,
    expect: "tecBYTECODE_REJECTED",
  })
  expectEscrowSurvived(responseSwap2Phase1, "atomic_swap2 phase 1")

  // atomic_swap2 phase 2 consumes Bob's escrow.
  const responseSwap2Phase2 = await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: swap2Result.sequence,
  })
  expectEscrowConsumed(responseSwap2Phase2, "atomic_swap2 phase 2")

  // Now atomic_swap1 must reject: counterpart is gone.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: swap1Result.sequence,
    expect: "tecBYTECODE_REJECTED",
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("counterpart_escrow"),
          MemoData: swap2Result.escrowId,
        },
      },
    ],
  })

  // ── Attempt 2: correct order — atomic_swap1 phase 1 BEFORE atomic_swap2 ────
  //     phase 2. Both eventually finish.

  const finalSwap1Result = await deploy(sourceWallet, destWallet, finish)
  const finalSwap2Result = await deploy(
    destWallet,
    sourceWallet,
    atomicSwap2Wasm,
    { Data: finalSwap1Result.escrowId },
  )

  // atomic_swap2 phase 1 (tecBYTECODE_REJECTED, escrow survives).
  await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: finalSwap2Result.sequence,
    expect: "tecBYTECODE_REJECTED",
  })

  // atomic_swap1 phase 1 BEFORE atomic_swap2 phase 2 (counterpart alive).
  const responseFinalSwap1Phase1 = await finishEscrow(
    testContext,
    sourceWallet,
    {
      Owner: sourceWallet.address,
      OfferSequence: finalSwap1Result.sequence,
      expect: "tecBYTECODE_REJECTED",
      Memos: [
        {
          Memo: {
            MemoType: xrpl.convertStringToHex("counterpart_escrow"),
            MemoData: finalSwap2Result.escrowId,
          },
        },
      ],
    },
  )
  expectEscrowSurvived(responseFinalSwap1Phase1, "atomic_swap1 phase 1")

  // atomic_swap1 phase 2 — consumes Alice's escrow.
  const responseFinalSwap1Phase2 = await finishEscrow(
    testContext,
    sourceWallet,
    {
      Owner: sourceWallet.address,
      OfferSequence: finalSwap1Result.sequence,
    },
  )
  expectEscrowConsumed(responseFinalSwap1Phase2, "atomic_swap1 phase 2")

  // atomic_swap2 phase 2 — consumes Bob's escrow.
  const responseFinalSwap2Phase2 = await finishEscrow(testContext, destWallet, {
    Owner: destWallet.address,
    OfferSequence: finalSwap2Result.sequence,
  })
  expectEscrowConsumed(responseFinalSwap2Phase2, "atomic_swap2 phase 2")
}

module.exports = { test }
