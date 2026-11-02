const xrpl = require("xrpl")

async function test(testContext) {
  const {
    deploy,
    finish,
    submit,
    sourceWallet,
    destWallet,
    finishEscrow,
    expectResult,
  } = testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // Mint an NFT owned by sourceWallet.
  const mintResponse = await submit(
    {
      TransactionType: "NFTokenMint",
      Account: sourceWallet.address,
      NFTokenTaxon: 0,
      URI: xrpl.convertStringToHex("https://example.com/nft-metadata.json"),
      Flags: xrpl.NFTokenMintFlags.tfTransferable,
    },
    sourceWallet,
  )
  expectResult(mintResponse, "tesSUCCESS", "NFTokenMint")
  const nftId = mintResponse.result.meta.nftoken_id

  const nftIdMemo = [
    {
      Memo: {
        MemoType: xrpl.convertStringToHex("nft_id"),
        MemoData: nftId,
      },
    },
  ]

  // destWallet does not yet own the NFT — finish must reject.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
    Memos: nftIdMemo,
    expect: "tecBYTECODE_REJECTED",
  })

  // Transfer the NFT via an NFTokenCreateOffer + NFTokenAcceptOffer.
  const offerResponse = await submit(
    {
      TransactionType: "NFTokenCreateOffer",
      Account: sourceWallet.address,
      NFTokenID: nftId,
      Amount: "0",
      Destination: destWallet.address,
      Flags: xrpl.NFTokenCreateOfferFlags.tfSellNFToken,
    },
    sourceWallet,
  )
  expectResult(offerResponse, "tesSUCCESS", "NFTokenCreateOffer")
  const nftOfferId = offerResponse.result.meta.AffectedNodes.find(
    (node) =>
      node.CreatedNode && node.CreatedNode.LedgerEntryType === "NFTokenOffer",
  ).CreatedNode.LedgerIndex

  expectResult(
    await submit(
      {
        TransactionType: "NFTokenAcceptOffer",
        Account: destWallet.address,
        NFTokenSellOffer: nftOfferId,
      },
      destWallet,
    ),
    "tesSUCCESS",
    "NFTokenAcceptOffer",
  )

  // destWallet now owns it — finish succeeds.
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: escrowResult.sequence,
    Memos: nftIdMemo,
  })
}

module.exports = { test }
