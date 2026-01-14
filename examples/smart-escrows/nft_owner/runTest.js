const xrpl = require("xrpl")

async function test(testContext) {
  const { deploy, finish, client, submit, sourceWallet, destWallet } =
    testContext

  const escrowResult = await deploy(sourceWallet, destWallet, finish)

  // Mint NFT
  const nftMint = {
    TransactionType: "NFTokenMint",
    Account: sourceWallet.address,
    NFTokenTaxon: 0,
    URI: xrpl.convertStringToHex("https://example.com/nft-metadata.json"),
    Flags: xrpl.NFTokenMintFlags.tfTransferable,
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

  // This EscrowFinish should fail because the destinationWallet is not the owner of the NFT
  const txFail = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
    Memos: [
      {
        Memo: {
          MemoType: xrpl.convertStringToHex("nft_id"),
          MemoData: nftId,
        },
      },
    ],
  }

  const responseFail = await submit(txFail, sourceWallet)

  if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.log("\nEscrow finished successfully????")
    process.exit(1)
  }

  // Transfer the NFT to the destinationWallet
  const nftOffer = {
    TransactionType: "NFTokenCreateOffer",
    Account: sourceWallet.address,
    NFTokenID: nftId,
    Amount: "0",
    Destination: destWallet.address,
    Flags: xrpl.NFTokenCreateOfferFlags.tfSellNFToken,
  }
  const offerResponse = await submit(nftOffer, sourceWallet)
  if (offerResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to create NFT offer:",
      offerResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }
  const nftOfferId = offerResponse.result.meta.AffectedNodes.find(
    (node) =>
      node.CreatedNode && node.CreatedNode.LedgerEntryType === "NFTokenOffer",
  ).CreatedNode.LedgerIndex

  const acceptOffer = {
    TransactionType: "NFTokenAcceptOffer",
    Account: destWallet.address,
    NFTokenSellOffer: nftOfferId,
  }
  const acceptResponse = await submit(acceptOffer, destWallet)
  if (acceptResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to accept NFT offer:",
      acceptResponse.result.meta.TransactionResult,
    )
    await client.disconnect()
    process.exit(1)
  }

  // This EscrowFinish should succeed because the destinationWallet is now the owner of the NFT
  const tx = {
    TransactionType: "EscrowFinish",
    Account: sourceWallet.address,
    Owner: sourceWallet.address,
    OfferSequence: parseInt(escrowResult.sequence),
    ComputationAllowance: 1000000,
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
    await client.disconnect()
    process.exit(1)
  }
}

module.exports = { test }
