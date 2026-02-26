const xrpl = require("xrpl")

const notary = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
  algorithm: xrpl.ECDSA.secp256k1,
})

async function test(testContext) {
  const { submit, sourceWallet, deploy, finish, destWallet } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  const txFail = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  // Submitting EscrowFinish transaction...
  // This should fail since the notary isn't sending this transaction
  const responseFail = await submit(txFail, sourceWallet)

  if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.log("\nEscrow finished successfully????")
    process.exit(1)
  }

  const tx = {
    TransactionType: "EscrowFinish",
    Account: notary.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  // Submitting EscrowFinish transaction...
  const response = await submit(tx, notary)

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
