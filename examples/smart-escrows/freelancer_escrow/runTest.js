const { TransactionType } = require("ripple-binary-codec/dist/enums");
const { decodeAccountID } = require("xrpl");

async function test(testContext) {
    const { client, submit, finish, sourceWallet, destWallet, fundWallet } = testContext;
    const arbWallet = await fundWallet();
    console.log(`Arbitrator test account: ${arbWallet.address}`)

    const ledger = await client.request({ command: "ledger", ledger_index: "validated" });
    const close_time = ledger.result.ledger.close_time;

    const buf = Buffer.alloc(28);
    const deadline = close_time + (30 * 60);

    buf.set(decodeAccountID(arbWallet.address));
    buf.writeUInt32LE(deadline, 20);
    buf[24] = 0;
    buf[25] = 0;
    buf[26] = 0;
    buf[27] = 0;
    const data = buf.toString("hex");

    const createRes = await submit({
        TransactionType: "EscrowCreate",
        Account: sourceWallet.address,
        Amount: "500000",
        Destination: destWallet.address,
        CancelAfter: close_time + 3600,
        FinishFunction: finish,
        Data: data
    })
}