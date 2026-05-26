const { decodeAccountID, convertStringToHex } = require("xrpl");

const INTENT_CONFIRM = 0;
const INTENT_DISPUTE = 2;

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
  };
}

async function test(testContext) {
  const { client, submit, finish, sourceWallet, destWallet, fundWallet } =
    testContext;
  const arbWallet = await fundWallet();
  console.log(`Arbitrator: ${arbWallet.address}`);

  const ledger = await client.request({
    command: "ledger",
    ledger_index: "validated",
  });
  const close_time = ledger.result.ledger.close_time;

  // 28-byte Data layout:
  //  0..20  arbitrator AccountID
  // 20..24  deadline in u32 LE, Ripple epoch seconds
  // 24..28  confirm/dispute state flags
  const buf = Buffer.alloc(28);
  buf.set(decodeAccountID(arbWallet.address));
  buf.writeUInt32LE(close_time + 30 * 60, 20);
  const data = buf.toString("hex");

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
    );
    if (res.result.meta.TransactionResult !== "tesSUCCESS") {
      console.error("EscrowCreate failed:", res.result.meta.TransactionResult);
      process.exit(1);
    }
    return res.result.tx_json.Sequence;
  }

  // Happy path
  console.log("\nBoth parties confirm");
  const seq1 = await createEscrow(data);
  // don't let arb / random wallet do confirm side effects
  const randomConfirm = await submit(
    escrowFinishTx(arbWallet.address, sourceWallet.address, seq1, INTENT_CONFIRM),
    arbWallet,
  );
  if (randomConfirm.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected invalid wallet to be refused confirmation, got:",
      randomConfirm.result.meta.TransactionResult,
    );
    process.exit(1);
  }

  // Freelancer confirms. Should get rejected as client hasn't confirmed.
  const freelancerConfirm = await submit(
    escrowFinishTx(destWallet.address, sourceWallet.address, seq1, INTENT_CONFIRM),
    destWallet,
  );
  if (freelancerConfirm.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error(
      "Expected waiting for client after freelancer confirm, got:",
      freelancerConfirm.result.meta.TransactionResult,
    );
    process.exit(1);
  }

  // Both confirm and escrow release
  const bothConfirm = await submit(
    escrowFinishTx(sourceWallet.address, sourceWallet.address, seq1, INTENT_CONFIRM),
    sourceWallet,
  );
  if (bothConfirm.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "Expected release after both confirm, got:",
      bothConfirm.result.meta.TransactionResult,
    );
    process.exit(1);
  }
}

module.exports = { test };
