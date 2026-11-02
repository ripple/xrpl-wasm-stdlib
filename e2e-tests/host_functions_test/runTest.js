const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, submit, sourceWallet, finish, finishEscrow, expectResult } =
    testContext

  // Mint an NFT with distinctive fields so the WASM contract can assert typed getters.
  const nftMint = {
    TransactionType: "NFTokenMint",
    Account: sourceWallet.address,
    NFTokenTaxon: 12345,
    TransferFee: 1000,
    URI: xrpl.convertStringToHex("https://example.com/nft-metadata.json"),
    Flags:
      xrpl.NFTokenMintFlags.tfBurnable |
      xrpl.NFTokenMintFlags.tfOnlyXRP |
      xrpl.NFTokenMintFlags.tfTransferable,
  }
  const mintResponse = await submit(nftMint, sourceWallet)
  expectResult(mintResponse, "tesSUCCESS", "NFTokenMint")
  const nftId = mintResponse.result.meta.nftoken_id

  // This escrow should always succeed. If it fails, something in rippled
  // is broken.
  const { sequence } = await deploy(sourceWallet, sourceWallet, finish)
  await finishEscrow(testContext, sourceWallet, {
    Owner: sourceWallet.address,
    OfferSequence: sequence,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("nft_id"),
          MemoData: nftId,
        },
      },
    ],
  })
}

module.exports = { test }
