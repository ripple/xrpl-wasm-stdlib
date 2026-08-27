const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, submit, sourceWallet, finish } = testContext

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
  if (mintResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to mint NFT:",
      mintResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  const nftId = mintResponse.result.meta.nftoken_id

  // This escrow should always succeed
  // If it fails, something in rippled is broken
  const { sequence } = await deploy(sourceWallet, sourceWallet, finish)
  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: sequence,
    Gas: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("nft_id"),
          MemoData: nftId,
        },
      },
    ],
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
