const { decodeAccountID, convertStringToHex } = require("xrpl")

const INTENT_CONFIRM = 0
const INTENT_DECONFIRM = 1
const INTENT_DISPUTE = 2
const INTENT_UNDISPUTE = 3

const ARB_RULE_FREELANCER = INTENT_CONFIRM
const ARB_RULE_CLIENT = INTENT_DISPUTE

function intentMemo(intent) {
  return [
    {
      Memo: {
        MemoType: convertStringToHex("intent"),
        MemoData: intent.toString(16).padStart(2, "0"),
      },
    },
  ]
}

async function test(testContext) {
  const {
    deploy,
    finish,
    sourceWallet,
    destWallet,
    fundWallet,
    finishEscrow,
    getLedgerCloseTime,
    client,
  } = testContext

  const arbWallet = await fundWallet()
  console.log(`Arbitrator: ${arbWallet.address}`)

  const close_time = await getLedgerCloseTime(client)

  // 27-byte Data layout:
  //  0..20  arbitrator AccountID
  // 20..24  deadline in u32 LE, Ripple epoch seconds
  // 24..27  confirm/dispute state flags
  const buf = Buffer.alloc(27)
  buf.set(decodeAccountID(arbWallet.address))
  buf.writeUInt32LE(close_time + 30 * 60, 20)
  const data = buf.toString("hex")

  const createEscrow = (escrowData) =>
    deploy(sourceWallet, destWallet, finish, {
      Data: escrowData,
      Amount: "500000",
      cancelAfterOffset: 3600,
    }).then((r) => r.sequence)

  // Local shorthand: submit an EscrowFinish carrying an intent memo.
  const finishWithIntent = (sender, seq, intent, expect) =>
    finishEscrow(testContext, sender, {
      Owner: sourceWallet.address,
      OfferSequence: seq,
      Memos: intentMemo(intent),
      expect,
    })

  // === Happy path: both parties confirm ===
  console.log("\nBoth parties confirm")
  const seq1 = await createEscrow(data)

  await finishWithIntent(
    arbWallet,
    seq1,
    INTENT_CONFIRM,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq1,
    INTENT_CONFIRM,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq1,
    INTENT_DECONFIRM,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq1,
    INTENT_CONFIRM,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(sourceWallet, seq1, INTENT_CONFIRM, "tesSUCCESS")

  // === Dispute path: arbitrator rules for freelancer ===
  console.log("\n--- Dispute path: arbitrator resolves for freelancer ---")
  const seq2 = await createEscrow(data)
  await finishWithIntent(
    sourceWallet,
    seq2,
    INTENT_DISPUTE,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq2,
    INTENT_DISPUTE,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(arbWallet, seq2, ARB_RULE_FREELANCER, "tesSUCCESS")

  // === Deadline auto-release: freelancer confirms past deadline ===
  console.log("\n--- Deadline path: freelancer confirms past deadline ---")
  const pastBuf = Buffer.alloc(27)
  pastBuf.set(decodeAccountID(arbWallet.address))
  pastBuf.writeUInt32LE(close_time - 1, 20)
  const seq3 = await createEscrow(pastBuf.toString("hex"))
  await finishWithIntent(destWallet, seq3, INTENT_CONFIRM, "tesSUCCESS")

  // === Arbitrator rules for client — escrow locked until CancelAfter ===
  console.log("\n--- Arbitrator rules for client ---")
  const seq4 = await createEscrow(data)
  await finishWithIntent(
    destWallet,
    seq4,
    INTENT_DISPUTE,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    arbWallet,
    seq4,
    ARB_RULE_CLIENT,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq4,
    INTENT_DISPUTE,
    "tecBYTECODE_REJECTED",
  )

  // === Self-resolve: disputing party withdraws ===
  console.log("\n--- Self-resolve: disputing party withdraws ---")
  const seq5 = await createEscrow(data)
  await finishWithIntent(
    sourceWallet,
    seq5,
    INTENT_DISPUTE,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq5,
    INTENT_UNDISPUTE,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    sourceWallet,
    seq5,
    INTENT_UNDISPUTE,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(
    destWallet,
    seq5,
    INTENT_CONFIRM,
    "tecBYTECODE_REJECTED",
  )
  await finishWithIntent(sourceWallet, seq5, INTENT_CONFIRM, "tesSUCCESS")

  // === Late dispute: deadline + freelancer_confirmed → release ===
  console.log(
    "\n--- Late dispute: deadline + freelancer_confirmed → release ---",
  )
  const lateDisputeBuf = Buffer.alloc(27)
  lateDisputeBuf.set(decodeAccountID(arbWallet.address))
  lateDisputeBuf.writeUInt32LE(close_time - 1, 20)
  lateDisputeBuf[25] = 1 // freelancer_confirmed=1
  const seq6 = await createEscrow(lateDisputeBuf.toString("hex"))
  await finishWithIntent(sourceWallet, seq6, INTENT_DISPUTE, "tesSUCCESS")
}

module.exports = { test }
