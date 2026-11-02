const xrpl = require("xrpl")

const oracleWallet = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", {
  algorithm: xrpl.ECDSA.secp256k1,
})

async function test(testContext) {
  const {
    deploy,
    finish,
    client,
    submit,
    sourceWallet,
    destWallet,
    finishEscrow,
    expectResult,
    getLedgerCloseTimeIso,
  } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  const closeTime = await getLedgerCloseTimeIso(client)

  // Publish an oracle reading of 1 USD/XRP.
  expectResult(
    await submit(
      {
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
      },
      oracleWallet,
    ),
    "tesSUCCESS",
    "OracleSet (create, price=1)",
  )

  // Contract requires price > 1 → escrow must reject.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
    expect: "tecBYTECODE_REJECTED",
  })

  // Update to price=2 USD/XRP.
  const closeTime2 = await getLedgerCloseTimeIso(client)
  expectResult(
    await submit(
      {
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
      },
      oracleWallet,
    ),
    "tesSUCCESS",
    "OracleSet (update, price=2)",
  )

  // Escrow now succeeds.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
  })
}

module.exports = { test }
