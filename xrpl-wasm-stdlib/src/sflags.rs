// Find a way to auto generate these from the ripple(d) source code

#![allow(non_upper_case_globals)]

// Universal Transaction flags:
pub const tfFullyCanonicalSig: u32 = 0x80000000;
pub const tfInnerBatchTxn: u32 = 0x40000000;
pub const tfUniversal: u32 = tfFullyCanonicalSig | tfInnerBatchTxn;
pub const tfUniversalMask: u32 = !tfUniversal;

// AccountSet flags:
pub const tfRequireDestTag: u32 = 0x00010000;
pub const tfOptionalDestTag: u32 = 0x00020000;
pub const tfRequireAuth: u32 = 0x00040000;
pub const tfOptionalAuth: u32 = 0x00080000;
pub const tfDisallowXRP: u32 = 0x00100000;
pub const tfAllowXRP: u32 = 0x00200000;
pub const tfAccountSetMask: u32 = !(tfUniversal
    | tfRequireDestTag
    | tfOptionalDestTag
    | tfRequireAuth
    | tfOptionalAuth
    | tfDisallowXRP
    | tfAllowXRP);

// AccountSet SetFlag/ClearFlag values
pub const asfRequireDest: u32 = 1;
pub const asfRequireAuth: u32 = 2;
pub const asfDisallowXRP: u32 = 3;
pub const asfDisableMaster: u32 = 4;
pub const asfAccountTxnID: u32 = 5;
pub const asfNoFreeze: u32 = 6;
pub const asfGlobalFreeze: u32 = 7;
pub const asfDefaultRipple: u32 = 8;
pub const asfDepositAuth: u32 = 9;
pub const asfAuthorizedNFTokenMinter: u32 = 10;
pub const asfDisallowIncomingNFTokenOffer: u32 = 12;
pub const asfDisallowIncomingCheck: u32 = 13;
pub const asfDisallowIncomingPayChan: u32 = 14;
pub const asfDisallowIncomingTrustline: u32 = 15;
pub const asfAllowTrustLineClawback: u32 = 16;
pub const asfAllowTrustLineLocking: u32 = 17;

// OfferCreate flags:
pub const tfPassive: u32 = 0x00010000;
pub const tfImmediateOrCancel: u32 = 0x00020000;
pub const tfFillOrKill: u32 = 0x00040000;
pub const tfSell: u32 = 0x00080000;
pub const tfHybrid: u32 = 0x00100000;
pub const tfOfferCreateMask: u32 =
    !(tfUniversal | tfPassive | tfImmediateOrCancel | tfFillOrKill | tfSell | tfHybrid);

// Payment flags:
pub const tfNoRippleDirect: u32 = 0x00010000;
pub const tfPartialPayment: u32 = 0x00020000;
pub const tfLimitQuality: u32 = 0x00040000;
pub const tfPaymentMask: u32 =
    !(tfUniversal | tfPartialPayment | tfLimitQuality | tfNoRippleDirect);
pub const tfMPTPaymentMask: u32 = !(tfUniversal | tfPartialPayment);

// TrustSet flags:
pub const tfSetfAuth: u32 = 0x00010000;
pub const tfSetNoRipple: u32 = 0x00020000;
pub const tfClearNoRipple: u32 = 0x00040000;
pub const tfSetFreeze: u32 = 0x00100000;
pub const tfClearFreeze: u32 = 0x00200000;
pub const tfSetDeepFreeze: u32 = 0x00400000;
pub const tfClearDeepFreeze: u32 = 0x00800000;
pub const tfTrustSetMask: u32 = !(tfUniversal
    | tfSetfAuth
    | tfSetNoRipple
    | tfClearNoRipple
    | tfSetFreeze
    | tfClearFreeze
    | tfSetDeepFreeze
    | tfClearDeepFreeze);
pub const tfTrustSetPermissionMask: u32 = !(tfUniversal | tfSetfAuth | tfSetFreeze | tfClearFreeze);

// EnableAmendment flags:
pub const tfGotMajority: u32 = 0x00010000;
pub const tfLostMajority: u32 = 0x00020000;
pub const tfChangeMask: u32 = !(tfUniversal | tfGotMajority | tfLostMajority);

// PaymentChannelClaim flags:
pub const tfRenew: u32 = 0x00010000;
pub const tfClose: u32 = 0x00020000;
pub const tfPayChanClaimMask: u32 = !(tfUniversal | tfRenew | tfClose);

// NFTokenMint flags:
pub const tfBurnable: u32 = 0x00000001;
pub const tfOnlyXRP: u32 = 0x00000002;
pub const tfTrustLine: u32 = 0x00000004;
pub const tfTransferable: u32 = 0x00000008;
pub const tfMutable: u32 = 0x00000010;

// MPTokenIssuanceCreate flags (values from LedgerFormats.h lsfMPT* constants):
pub const tfMPTCanLock: u32 = 0x00000002;
pub const tfMPTRequireAuth: u32 = 0x00000004;
pub const tfMPTCanEscrow: u32 = 0x00000008;
pub const tfMPTCanTrade: u32 = 0x00000010;
pub const tfMPTCanTransfer: u32 = 0x00000020;
pub const tfMPTCanClawback: u32 = 0x00000040;
pub const tfMPTokenIssuanceCreateMask: u32 = !(tfUniversal
    | tfMPTCanLock
    | tfMPTRequireAuth
    | tfMPTCanEscrow
    | tfMPTCanTrade
    | tfMPTCanTransfer
    | tfMPTCanClawback);

// MPTokenIssuanceCreate MutableFlags (values from LedgerFormats.h lsmfMPT* constants):
pub const tmfMPTCanMutateCanLock: u32 = 0x00000002;
pub const tmfMPTCanMutateRequireAuth: u32 = 0x00000004;
pub const tmfMPTCanMutateCanEscrow: u32 = 0x00000008;
pub const tmfMPTCanMutateCanTrade: u32 = 0x00000010;
pub const tmfMPTCanMutateCanTransfer: u32 = 0x00000020;
pub const tmfMPTCanMutateCanClawback: u32 = 0x00000040;
pub const tmfMPTCanMutateMetadata: u32 = 0x00010000;
pub const tmfMPTCanMutateTransferFee: u32 = 0x00020000;
pub const tmfMPTokenIssuanceCreateMutableMask: u32 = !(tmfMPTCanMutateCanLock
    | tmfMPTCanMutateRequireAuth
    | tmfMPTCanMutateCanEscrow
    | tmfMPTCanMutateCanTrade
    | tmfMPTCanMutateCanTransfer
    | tmfMPTCanMutateCanClawback
    | tmfMPTCanMutateMetadata
    | tmfMPTCanMutateTransferFee);

// MPTokenAuthorize flags:
pub const tfMPTUnauthorize: u32 = 0x00000001;
pub const tfMPTokenAuthorizeMask: u32 = !(tfUniversal | tfMPTUnauthorize);

// MPTokenIssuanceSet flags:
pub const tfMPTLock: u32 = 0x00000001;
pub const tfMPTUnlock: u32 = 0x00000002;
pub const tfMPTokenIssuanceSetMask: u32 = !(tfUniversal | tfMPTLock | tfMPTUnlock);
pub const tfMPTokenIssuanceSetPermissionMask: u32 = !(tfUniversal | tfMPTLock | tfMPTUnlock);

// MPTokenIssuanceSet MutableFlags:
pub const tmfMPTSetCanLock: u32 = 0x00000001;
pub const tmfMPTClearCanLock: u32 = 0x00000002;
pub const tmfMPTSetRequireAuth: u32 = 0x00000004;
pub const tmfMPTClearRequireAuth: u32 = 0x00000008;
pub const tmfMPTSetCanEscrow: u32 = 0x00000010;
pub const tmfMPTClearCanEscrow: u32 = 0x00000020;
pub const tmfMPTSetCanTrade: u32 = 0x00000040;
pub const tmfMPTClearCanTrade: u32 = 0x00000080;
pub const tmfMPTSetCanTransfer: u32 = 0x00000100;
pub const tmfMPTClearCanTransfer: u32 = 0x00000200;
pub const tmfMPTSetCanClawback: u32 = 0x00000400;
pub const tmfMPTClearCanClawback: u32 = 0x00000800;
pub const tmfMPTokenIssuanceSetMutableMask: u32 = !(tmfMPTSetCanLock
    | tmfMPTClearCanLock
    | tmfMPTSetRequireAuth
    | tmfMPTClearRequireAuth
    | tmfMPTSetCanEscrow
    | tmfMPTClearCanEscrow
    | tmfMPTSetCanTrade
    | tmfMPTClearCanTrade
    | tmfMPTSetCanTransfer
    | tmfMPTClearCanTransfer
    | tmfMPTSetCanClawback
    | tmfMPTClearCanClawback);

// MPTokenIssuanceDestroy flags:
pub const tfMPTokenIssuanceDestroyMask: u32 = !tfUniversal;

// NFToken masks:
pub const tfNFTokenMintMask: u32 = !(tfUniversal | tfBurnable | tfOnlyXRP | tfTransferable);
pub const tfNFTokenMintOldMask: u32 = !(!tfNFTokenMintMask | tfTrustLine);
pub const tfNFTokenMintOldMaskWithMutable: u32 = !(!tfNFTokenMintOldMask | tfMutable);
pub const tfNFTokenMintMaskWithMutable: u32 = !(!tfNFTokenMintMask | tfMutable);

// NFTokenCreateOffer flags:
pub const tfSellNFToken: u32 = 0x00000001;
pub const tfNFTokenCreateOfferMask: u32 = !(tfUniversal | tfSellNFToken);

// NFTokenCancelOffer flags:
pub const tfNFTokenCancelOfferMask: u32 = !tfUniversal;

// NFTokenAcceptOffer flags:
pub const tfNFTokenAcceptOfferMask: u32 = !tfUniversal;

// Clawback flags:
pub const tfClawbackMask: u32 = !tfUniversal;

// AMM Flags:
pub const tfLPToken: u32 = 0x00010000;
pub const tfWithdrawAll: u32 = 0x00020000;
pub const tfOneAssetWithdrawAll: u32 = 0x00040000;
pub const tfSingleAsset: u32 = 0x00080000;
pub const tfTwoAsset: u32 = 0x00100000;
pub const tfOneAssetLPToken: u32 = 0x00200000;
pub const tfLimitLPToken: u32 = 0x00400000;
pub const tfTwoAssetIfEmpty: u32 = 0x00800000;
pub const tfWithdrawSubTx: u32 = tfLPToken
    | tfSingleAsset
    | tfTwoAsset
    | tfOneAssetLPToken
    | tfLimitLPToken
    | tfWithdrawAll
    | tfOneAssetWithdrawAll;
pub const tfDepositSubTx: u32 =
    tfLPToken | tfSingleAsset | tfTwoAsset | tfOneAssetLPToken | tfLimitLPToken | tfTwoAssetIfEmpty;
pub const tfWithdrawMask: u32 = !(tfUniversal | tfWithdrawSubTx);
pub const tfDepositMask: u32 = !(tfUniversal | tfDepositSubTx);

// AMMClawback flags:
pub const tfClawTwoAssets: u32 = 0x00000001;
pub const tfAMMClawbackMask: u32 = !(tfUniversal | tfClawTwoAssets);

// BridgeModify flags:
pub const tfClearAccountCreateAmount: u32 = 0x00010000;
pub const tfBridgeModifyMask: u32 = !(tfUniversal | tfClearAccountCreateAmount);

// VaultCreate flags:
pub const tfVaultPrivate: u32 = 0x00010000;
pub const tfVaultShareNonTransferable: u32 = 0x00020000;
pub const tfVaultCreateMask: u32 = !(tfUniversal | tfVaultPrivate | tfVaultShareNonTransferable);

// Batch Flags:
pub const tfAllOrNothing: u32 = 0x00010000;
pub const tfOnlyOne: u32 = 0x00020000;
pub const tfUntilFailure: u32 = 0x00040000;
pub const tfIndependent: u32 = 0x00080000;
pub const tfBatchMask: u32 =
    !(tfUniversal | tfAllOrNothing | tfOnlyOne | tfUntilFailure | tfIndependent) | tfInnerBatchTxn;

// Contract flags:
pub const tfImmutable: u32 = 0x00010000;
pub const tfCodeImmutable: u32 = 0x00020000;
pub const tfABIImmutable: u32 = 0x00040000;
pub const tfUndeletable: u32 = 0x00080000;
pub const tfContractMask: u32 =
    !(tfUniversal | tfImmutable | tfCodeImmutable | tfABIImmutable | tfUndeletable);

// Contract parameter flags:
pub const tfSendAmount: u32 = 0x00010000;
pub const tfSendNFToken: u32 = 0x00020000;
pub const tfAuthorizeToken: u32 = 0x00040000;
pub const tfContractParameterMask: u32 = !(tfSendAmount | tfSendNFToken | tfAuthorizeToken);
