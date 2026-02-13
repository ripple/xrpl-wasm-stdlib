const xrpl = require("xrpl")

const oracleWallet = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
  algorithm: xrpl.ECDSA.secp256k1,
})

async function test(testContext) {
  const { deploy, finish, client, submit, sourceWallet, destWallet } =
    testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  const closeTime = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time_iso

  const oracleCreate = {
    TransactionType: "OracleSet",
    Account: oracleWallet.address,
    OracleDocumentID: 1,
    Provider: xrpl.convertStringToHex("sample"),
    AssetClass: xrpl.convertStringToHex("currency"),
    LastUpdateTime: Math.floor(new Date(closeTime).getTime() / 1000) + 20,
    PriceDataSeries: [
      {
        PriceData: {
          BaseAsset: "XRP",
          QuoteAsset: "USD",
          AssetPrice: 1,
          Scale: 1,
        },
      },
    ],
  }
  const oracleCreateResponse = await submit(oracleCreate, oracleWallet)
  if (oracleCreateResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create oracle:",
      oracleCreateResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  const txFail = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  // This EscrowCreate should fail since the oracle must show the price as <= 1 USD/XRP
  const responseFail = await submit(txFail, sourceWallet)

  if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error("\nEscrow finished successfully when it should have failed")
    process.exit(1)
  }

  const closeTime2 = (
    await client.request({
      command: "ledger",
      ledger_index: "validated",
    })
  ).result.ledger.close_time_iso

  const oracleUpdate = {
    TransactionType: "OracleSet",
    Account: oracleWallet.address,
    OracleDocumentID: 1,
    LastUpdateTime: Math.floor(new Date(closeTime2).getTime() / 1000) + 20,
    PriceDataSeries: [
      {
        PriceData: {
          BaseAsset: "XRP",
          QuoteAsset: "USD",
          AssetPrice: 2,
          Scale: 1,
        },
      },
    ],
  }
  const oracleUpdateResponse = await submit(oracleUpdate, oracleWallet)
  if (oracleUpdateResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create oracle:",
      oracleUpdateResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
  }

  const response = await submit(tx, sourceWallet)

  if (response.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to finish escrow:",
      response.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
