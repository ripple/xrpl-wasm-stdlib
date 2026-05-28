const { decodeAccountID, convertStringToHex } = require("xrpl")

const INTENT_CONFIRM = 0
const INTENT_DECONFIRM = 1
const INTENT_DISPUTE = 2
const INTENT_UNDISPUTE = 3

const ARB_RULE_FREELANCER = INTENT_CONFIRM
const ARB_RULE_CLIENT = INTENT_DISPUTE

function escrowFinishTx(senderAddr, ownerAddr, sequence, intent) {
  return {
    TransactionType: "EscrowFinish",
    Account: senderAddr,
    Owner: ownerAddr,
    OfferSequence: parseInt(sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: convertStringToHex("intent"),
          MemoData: intent.toString(16).padStart(2, "0"),
        },
      },
    ],
  }
}

async function test(testContext) {
  const { client, submit, finish, sourceWallet, destWallet, fundWallet } =
    testContext
  const arbWallet = await fundWallet()
  console.log(`Arbitrator: ${arbWallet.address}`)

  const ledger = await client.request({
    command: "ledger",
    ledger_index: "validated",
  })
  const close_time = ledger.result.ledger.close_time

  // 27-byte Data layout:
  //  0..20  arbitrator AccountID
  // 20..24  deadline in u32 LE, Ripple epoch seconds
  // 24..27  confirm/dispute state flags
  const buf = Buffer.alloc(27)
  buf.set(decodeAccountID(arbWallet.address))
  buf.writeUInt32LE(close_time + 30 * 60, 20)
  const data = buf.toString("hex")

  async function createEscrow(escrowData) {
    const res = await submit(
      {
        TransactionType: "EscrowCreate",
        Account: sourceWallet.address,
        Amount: "500000",
        Destination: destWallet.address,
        CancelAfter: close_time + 3600,
        FinishFunction: finish,
        Data: escrowData,
      },
      sourceWallet,
    )
    if (res.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error("EscrowCreate failed:", res.result.meta.TransactionResult)
      process.exit(1)
    }
    return res.result.tx_json.Sequence
  }

  // Happy path
  console.log("\nBoth parties confirm")
  const seq1 = await createEscrow(data)
  // don't let arb / random wallet do confirm side effects
  const randomConfirm = await submit(
    escrowFinishTx(
      arbWallet.address,
      sourceWallet.address,
      seq1,
      INTENT_CONFIRM,
    ),
    arbWallet,
  )
  if (randomConfirm.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected invalid wallet to be refused confirmation, got:",
      randomConfirm.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Freelancer confirms. Should get rejected as client hasn't confirmed.
  const freelancerConfirm = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq1,
      INTENT_CONFIRM,
    ),
    destWallet,
  )
  if (freelancerConfirm.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected waiting for client after freelancer confirm, got:",
      freelancerConfirm.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Freelancer deconfirms — FREELANCER_CONFIRMED back to 0.
  const freelancerDeconfirm = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq1,
      INTENT_DECONFIRM,
    ),
    destWallet,
  )
  if (
    freelancerDeconfirm.result.meta.TransactionResult !== "tecWASM_REJECTED"
  ) {
    console.error(
      "Expected hold after freelancer deconfirm, got:",
      freelancerDeconfirm.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Freelancer re-confirms.
  const freelancerReconfirm = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq1,
      INTENT_CONFIRM,
    ),
    destWallet,
  )
  if (
    freelancerReconfirm.result.meta.TransactionResult !== "tecWASM_REJECTED"
  ) {
    console.error(
      "Expected hold after freelancer re-confirm, got:",
      freelancerReconfirm.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Both confirm and escrow release
  const bothConfirm = await submit(
    escrowFinishTx(
      sourceWallet.address,
      sourceWallet.address,
      seq1,
      INTENT_CONFIRM,
    ),
    sourceWallet,
  )
  if (bothConfirm.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Expected release after both confirm, got:",
      bothConfirm.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // === Dispute path — arbitrator rules for freelancer ===
  console.log("\n--- Dispute path: arbitrator resolves for freelancer ---")
  const seq2 = await createEscrow(data)

  // Client raises a dispute — clears confirm flags, sets DISPUTING_PARTY=client.
  const raiseDispute = await submit(
    escrowFinishTx(
      sourceWallet.address,
      sourceWallet.address,
      seq2,
      INTENT_DISPUTE,
    ),
    sourceWallet,
  )
  if (raiseDispute.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected hold after raising dispute, got:",
      raiseDispute.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Freelancer tries to resolve the client's dispute — only the disputing party can withdraw.
  const wrongResolver = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq2,
      INTENT_DISPUTE,
    ),
    destWallet,
  )
  if (wrongResolver.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected non-disputer resolution to be rejected, got:",
      wrongResolver.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Arbitrator rules in favor of the freelancer — escrow releases.
  const arbResolve = await submit(
    escrowFinishTx(
      arbWallet.address,
      sourceWallet.address,
      seq2,
      ARB_RULE_FREELANCER,
    ),
    arbWallet,
  )
  if (arbResolve.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Expected arbitrator to release escrow, got:",
      arbResolve.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // === Deadline auto-release — freelancer confirms past deadline ===
  console.log("\n--- Deadline path: freelancer confirms past deadline ---")
  const pastBuf = Buffer.alloc(27)
  pastBuf.set(decodeAccountID(arbWallet.address))
  pastBuf.writeUInt32LE(close_time - 1, 20) // deadline already in the past
  const dataPast = pastBuf.toString("hex")
  const seq3 = await createEscrow(dataPast)

  // Freelancer confirms alone — past deadline + freelancer_confirmed = release.
  const deadlineRelease = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq3,
      INTENT_CONFIRM,
    ),
    destWallet,
  )
  if (deadlineRelease.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Expected deadline auto-release, got:",
      deadlineRelease.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // === Escrow 4: Arbitrator rules for client — escrow locked until CancelAfter ===
  console.log("\n--- Arbitrator rules for client ---")
  const seq4 = await createEscrow(data)

  // Freelancer raises a dispute.
  const raiseDispute2 = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq4,
      INTENT_DISPUTE,
    ),
    destWallet,
  )
  if (raiseDispute2.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected hold after raising dispute, got:",
      raiseDispute2.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Arbitrator rules for client — escrow locked, DISPUTING_PARTY set to ARB_LOCK.
  const arbRuleClient = await submit(
    escrowFinishTx(
      arbWallet.address,
      sourceWallet.address,
      seq4,
      ARB_RULE_CLIENT,
    ),
    arbWallet,
  )
  if (arbRuleClient.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected lock after arb rules for client, got:",
      arbRuleClient.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Freelancer tries to finish after the arb ruling — should be blocked.
  const freelancerBlocked = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq4,
      INTENT_DISPUTE,
    ),
    destWallet,
  )
  if (freelancerBlocked.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected freelancer to be blocked after arb ruling, got:",
      freelancerBlocked.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // === Escrow 5: Self-resolve — disputing party withdraws with INTENT_UNDISPUTE ===
  console.log("\n--- Self-resolve: disputing party withdraws ---")
  const seq5 = await createEscrow(data)

  // Client raises a dispute.
  const raiseDispute3 = await submit(
    escrowFinishTx(
      sourceWallet.address,
      sourceWallet.address,
      seq5,
      INTENT_DISPUTE,
    ),
    sourceWallet,
  )
  if (raiseDispute3.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected hold after raising dispute, got:",
      raiseDispute3.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Freelancer tries to undispute the client's dispute — only the disputing party can withdraw.
  const wrongUndispute = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq5,
      INTENT_UNDISPUTE,
    ),
    destWallet,
  )
  if (wrongUndispute.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected non-disputer undispute to be rejected, got:",
      wrongUndispute.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Client withdraws their own dispute — back to pending.
  const selfResolve = await submit(
    escrowFinishTx(
      sourceWallet.address,
      sourceWallet.address,
      seq5,
      INTENT_UNDISPUTE,
    ),
    sourceWallet,
  )
  if (selfResolve.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected hold after self-resolve (back to pending), got:",
      selfResolve.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // Both confirm — escrow releases now that dispute is cleared.
  const confirmAfterResolve = await submit(
    escrowFinishTx(
      destWallet.address,
      sourceWallet.address,
      seq5,
      INTENT_CONFIRM,
    ),
    destWallet,
  )
  if (
    confirmAfterResolve.result.meta.TransactionResult !== "tecWASM_REJECTED"
  ) {
    console.error(
      "Expected hold waiting for client after freelancer confirm, got:",
      confirmAfterResolve.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  const releaseAfterResolve = await submit(
    escrowFinishTx(
      sourceWallet.address,
      sourceWallet.address,
      seq5,
      INTENT_CONFIRM,
    ),
    sourceWallet,
  )
  if (releaseAfterResolve.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Expected release after both confirm post-resolve, got:",
      releaseAfterResolve.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
