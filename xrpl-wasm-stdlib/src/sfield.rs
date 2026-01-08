#![allow(non_upper_case_globals)]

use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::core::ledger_objects::array_object::{Array, Object};
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::core::types::blob::{Blob, DEFAULT_BLOB_SIZE, DOMAIN_BLOB_SIZE, MEMO_BLOB_SIZE};
use crate::core::types::currency::Currency;
use crate::core::types::issue::Issue;
use crate::core::types::public_key::PUBLIC_KEY_BUFFER_SIZE;
use crate::core::types::uint::{Hash128, Hash160, Hash192, Hash256};
use core::marker::PhantomData;

/// A type-safe wrapper for XRPL serialized field codes.
///
/// This struct encodes both the field code and the expected type as const generics,
/// allowing the compiler to automatically infer the correct type when calling `get_field`.
///
/// # Example
///
/// ```rust,no_run
/// use xrpl_wasm_stdlib::core::ledger_objects::ledger_object;
/// use xrpl_wasm_stdlib::sfield;
///
/// // Type is automatically inferred from the SField constant
/// let flags = ledger_object::get_field(0, sfield::Flags).unwrap();  // u32
/// let balance = ledger_object::get_field(0, sfield::Balance).unwrap();  // u64
/// ```
#[derive(Copy, Clone)]
pub struct SField<T: LedgerObjectFieldGetter, const CODE: i32> {
    _phantom: PhantomData<T>,
}

impl<T: LedgerObjectFieldGetter, const CODE: i32> SField<T, CODE> {
    /// Creates a new SField constant.
    ///
    /// This is a const function that can be used to initialize SField constants.
    pub const fn new() -> Self {
        SField {
            _phantom: PhantomData,
        }
    }
}

impl<T: LedgerObjectFieldGetter, const CODE: i32> From<SField<T, CODE>> for i32 {
    fn from(_: SField<T, CODE>) -> Self {
        CODE
    }
}

impl<T: LedgerObjectFieldGetter, const CODE: i32> Default for SField<T, CODE> {
    fn default() -> Self {
        Self::new()
    }
}

pub const Invalid: SField<u8, -1> = SField::new();
pub const Generic: SField<u8, 0> = SField::new();
pub const hash: SField<u8, -1> = SField::new();
pub const index: SField<u8, 0> = SField::new();

// Placeholder SField constants for array and object types
// These types don't have LedgerObjectFieldGetter implementations but are represented as SField<Array/Object, CODE>
pub const LedgerEntryType: SField<u16, 65537> = SField::new();
pub const TransactionType: SField<u16, 65538> = SField::new();
pub const SignerWeight: SField<u16, 65539> = SField::new();
pub const TransferFee: SField<u16, 65540> = SField::new();
pub const TradingFee: SField<u16, 65541> = SField::new();
pub const DiscountedFee: SField<u16, 65542> = SField::new();
pub const Version: SField<u16, 65552> = SField::new();
pub const HookStateChangeCount: SField<u16, 65553> = SField::new();
pub const HookEmitCount: SField<u16, 65554> = SField::new();
pub const HookExecutionIndex: SField<u16, 65555> = SField::new();
pub const HookApiVersion: SField<u16, 65556> = SField::new();
pub const LedgerFixType: SField<u16, 65557> = SField::new();
pub const NetworkID: SField<u32, 131073> = SField::new();
pub const Flags: SField<u32, 131074> = SField::new();
pub const SourceTag: SField<u32, 131075> = SField::new();
pub const Sequence: SField<u32, 131076> = SField::new();
pub const PreviousTxnLgrSeq: SField<u32, 131077> = SField::new();
pub const LedgerSequence: SField<u32, 131078> = SField::new();
pub const CloseTime: SField<u32, 131079> = SField::new();
pub const ParentCloseTime: SField<u32, 131080> = SField::new();
pub const SigningTime: SField<u32, 131081> = SField::new();
pub const Expiration: SField<u32, 131082> = SField::new();
pub const TransferRate: SField<u32, 131083> = SField::new();
pub const WalletSize: SField<u32, 131084> = SField::new();
pub const OwnerCount: SField<u32, 131085> = SField::new();
pub const DestinationTag: SField<u32, 131086> = SField::new();
pub const LastUpdateTime: SField<u32, 131087> = SField::new();
pub const HighQualityIn: SField<u32, 131088> = SField::new();
pub const HighQualityOut: SField<u32, 131089> = SField::new();
pub const LowQualityIn: SField<u32, 131090> = SField::new();
pub const LowQualityOut: SField<u32, 131091> = SField::new();
pub const QualityIn: SField<u32, 131092> = SField::new();
pub const QualityOut: SField<u32, 131093> = SField::new();
pub const StampEscrow: SField<u32, 131094> = SField::new();
pub const BondAmount: SField<u32, 131095> = SField::new();
pub const LoadFee: SField<u32, 131096> = SField::new();
pub const OfferSequence: SField<u32, 131097> = SField::new();
pub const FirstLedgerSequence: SField<u32, 131098> = SField::new();
pub const LastLedgerSequence: SField<u32, 131099> = SField::new();
pub const TransactionIndex: SField<u32, 131100> = SField::new();
pub const OperationLimit: SField<u32, 131101> = SField::new();
pub const ReferenceFeeUnits: SField<u32, 131102> = SField::new();
pub const ReserveBase: SField<u32, 131103> = SField::new();
pub const ReserveIncrement: SField<u32, 131104> = SField::new();
pub const SetFlag: SField<u32, 131105> = SField::new();
pub const ClearFlag: SField<u32, 131106> = SField::new();
pub const SignerQuorum: SField<u32, 131107> = SField::new();
pub const CancelAfter: SField<u32, 131108> = SField::new();
pub const FinishAfter: SField<u32, 131109> = SField::new();
pub const SignerListID: SField<u32, 131110> = SField::new();
pub const SettleDelay: SField<u32, 131111> = SField::new();
pub const TicketCount: SField<u32, 131112> = SField::new();
pub const TicketSequence: SField<u32, 131113> = SField::new();
pub const NFTokenTaxon: SField<u32, 131114> = SField::new();
pub const MintedNFTokens: SField<u32, 131115> = SField::new();
pub const BurnedNFTokens: SField<u32, 131116> = SField::new();
pub const HookStateCount: SField<u32, 131117> = SField::new();
pub const EmitGeneration: SField<u32, 131118> = SField::new();
pub const VoteWeight: SField<u32, 131120> = SField::new();
pub const FirstNFTokenSequence: SField<u32, 131122> = SField::new();
pub const OracleDocumentID: SField<u32, 131123> = SField::new();
pub const PermissionValue: SField<u32, 131124> = SField::new();
pub const MutableFlags: SField<u32, 131125> = SField::new();
pub const ExtensionComputeLimit: SField<u32, 131126> = SField::new();
pub const ExtensionSizeLimit: SField<u32, 131127> = SField::new();
pub const GasPrice: SField<u32, 131128> = SField::new();
pub const ComputationAllowance: SField<u32, 131129> = SField::new();
pub const GasUsed: SField<u32, 131130> = SField::new();
pub const IndexNext: SField<u64, 196609> = SField::new();
pub const IndexPrevious: SField<u64, 196610> = SField::new();
pub const BookNode: SField<u64, 196611> = SField::new();
pub const OwnerNode: SField<u64, 196612> = SField::new();
pub const BaseFee: SField<u64, 196613> = SField::new();
pub const ExchangeRate: SField<u64, 196614> = SField::new();
pub const LowNode: SField<u64, 196615> = SField::new();
pub const HighNode: SField<u64, 196616> = SField::new();
pub const DestinationNode: SField<u64, 196617> = SField::new();
pub const Cookie: SField<u64, 196618> = SField::new();
pub const ServerVersion: SField<u64, 196619> = SField::new();
pub const NFTokenOfferNode: SField<u64, 196620> = SField::new();
pub const EmitBurden: SField<u64, 196621> = SField::new();
pub const HookOn: SField<u64, 196624> = SField::new();
pub const HookInstructionCount: SField<u64, 196625> = SField::new();
pub const HookReturnCode: SField<u64, 196626> = SField::new();
pub const ReferenceCount: SField<u64, 196627> = SField::new();
pub const XChainClaimID: SField<u64, 196628> = SField::new();
pub const XChainAccountCreateCount: SField<u64, 196629> = SField::new();
pub const XChainAccountClaimCount: SField<u64, 196630> = SField::new();
pub const AssetPrice: SField<u64, 196631> = SField::new();
pub const MaximumAmount: SField<u64, 196632> = SField::new();
pub const OutstandingAmount: SField<u64, 196633> = SField::new();
pub const MPTAmount: SField<u64, 196634> = SField::new();
pub const IssuerNode: SField<u64, 196635> = SField::new();
pub const SubjectNode: SField<u64, 196636> = SField::new();
pub const LockedAmount: SField<u64, 196637> = SField::new();
pub const EmailHash: SField<Hash128, 262145> = SField::new();
pub const LedgerHash: SField<Hash256, 327681> = SField::new();
pub const ParentHash: SField<Hash256, 327682> = SField::new();
pub const TransactionHash: SField<Hash256, 327683> = SField::new();
pub const AccountHash: SField<Hash256, 327684> = SField::new();
pub const PreviousTxnID: SField<Hash256, 327685> = SField::new();
pub const LedgerIndex: SField<Hash256, 327686> = SField::new();
pub const WalletLocator: SField<Hash256, 327687> = SField::new();
pub const RootIndex: SField<Hash256, 327688> = SField::new();
pub const AccountTxnID: SField<Hash256, 327689> = SField::new();
pub const NFTokenID: SField<Hash256, 327690> = SField::new();
pub const EmitParentTxnID: SField<Hash256, 327691> = SField::new();
pub const EmitNonce: SField<Hash256, 327692> = SField::new();
pub const EmitHookHash: SField<Hash256, 327693> = SField::new();
pub const AMMID: SField<Hash256, 327694> = SField::new();
pub const BookDirectory: SField<Hash256, 327696> = SField::new();
pub const InvoiceID: SField<Hash256, 327697> = SField::new();
pub const Nickname: SField<Hash256, 327698> = SField::new();
pub const Amendment: SField<Hash256, 327699> = SField::new();
pub const Digest: SField<Hash256, 327701> = SField::new();
pub const Channel: SField<Hash256, 327702> = SField::new();
pub const ConsensusHash: SField<Hash256, 327703> = SField::new();
pub const CheckID: SField<Hash256, 327704> = SField::new();
pub const ValidatedHash: SField<Hash256, 327705> = SField::new();
pub const PreviousPageMin: SField<Hash256, 327706> = SField::new();
pub const NextPageMin: SField<Hash256, 327707> = SField::new();
pub const NFTokenBuyOffer: SField<Hash256, 327708> = SField::new();
pub const NFTokenSellOffer: SField<Hash256, 327709> = SField::new();
pub const HookStateKey: SField<Hash256, 327710> = SField::new();
pub const HookHash: SField<Hash256, 327711> = SField::new();
pub const HookNamespace: SField<Hash256, 327712> = SField::new();
pub const HookSetTxnID: SField<Hash256, 327713> = SField::new();
pub const DomainID: SField<Hash256, 327714> = SField::new();
pub const VaultID: SField<Hash256, 327715> = SField::new();
pub const ParentBatchID: SField<Hash256, 327716> = SField::new();
pub const Amount: SField<Amount, 393217> = SField::new();
pub const Balance: SField<Amount, 393218> = SField::new();
pub const LimitAmount: SField<Amount, 393219> = SField::new();
pub const TakerPays: SField<Amount, 393220> = SField::new();
pub const TakerGets: SField<Amount, 393221> = SField::new();
pub const LowLimit: SField<Amount, 393222> = SField::new();
pub const HighLimit: SField<Amount, 393223> = SField::new();
pub const Fee: SField<Amount, 393224> = SField::new();
pub const SendMax: SField<Amount, 393225> = SField::new();
pub const DeliverMin: SField<Amount, 393226> = SField::new();
pub const Amount2: SField<Amount, 393227> = SField::new();
pub const BidMin: SField<Amount, 393228> = SField::new();
pub const BidMax: SField<Amount, 393229> = SField::new();
pub const MinimumOffer: SField<Amount, 393232> = SField::new();
pub const RippleEscrow: SField<Amount, 393233> = SField::new();
pub const DeliveredAmount: SField<Amount, 393234> = SField::new();
pub const NFTokenBrokerFee: SField<Amount, 393235> = SField::new();
pub const BaseFeeDrops: SField<Amount, 393238> = SField::new();
pub const ReserveBaseDrops: SField<Amount, 393239> = SField::new();
pub const ReserveIncrementDrops: SField<Amount, 393240> = SField::new();
pub const LPTokenOut: SField<Amount, 393241> = SField::new();
pub const LPTokenIn: SField<Amount, 393242> = SField::new();
pub const EPrice: SField<Amount, 393243> = SField::new();
pub const Price: SField<Amount, 393244> = SField::new();
pub const SignatureReward: SField<Amount, 393245> = SField::new();
pub const MinAccountCreateAmount: SField<Amount, 393246> = SField::new();
pub const LPTokenBalance: SField<Amount, 393247> = SField::new();
pub const PublicKey: SField<Blob<{ PUBLIC_KEY_BUFFER_SIZE }>, 458753> = SField::new();
pub const MessageKey: SField<Blob<{ PUBLIC_KEY_BUFFER_SIZE }>, 458754> = SField::new();
pub const SigningPubKey: SField<Blob<{ PUBLIC_KEY_BUFFER_SIZE }>, 458755> = SField::new();
pub const TxnSignature: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458756> = SField::new();
pub const URI: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458757> = SField::new();
pub const Signature: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458758> = SField::new();
pub const Domain: SField<Blob<{ DOMAIN_BLOB_SIZE }>, 458759> = SField::new();
pub const FundCode: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458760> = SField::new();
pub const RemoveCode: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458761> = SField::new();
pub const ExpireCode: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458762> = SField::new();
pub const CreateCode: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458763> = SField::new();
pub const MemoType: SField<Blob<{ MEMO_BLOB_SIZE }>, 458764> = SField::new();
pub const MemoData: SField<Blob<{ MEMO_BLOB_SIZE }>, 458765> = SField::new();
pub const MemoFormat: SField<Blob<{ MEMO_BLOB_SIZE }>, 458766> = SField::new();
pub const Fulfillment: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458768> = SField::new();
pub const Condition: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458769> = SField::new();
pub const MasterSignature: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458770> = SField::new();
pub const UNLModifyValidator: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458771> = SField::new();
pub const ValidatorToDisable: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458772> = SField::new();
pub const ValidatorToReEnable: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458773> = SField::new();
pub const HookStateData: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458774> = SField::new();
pub const HookReturnString: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458775> = SField::new();
pub const HookParameterName: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458776> = SField::new();
pub const HookParameterValue: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458777> = SField::new();
pub const DIDDocument: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458778> = SField::new();
pub const Data: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458779> = SField::new();
pub const AssetClass: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458780> = SField::new();
pub const Provider: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458781> = SField::new();
pub const MPTokenMetadata: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458782> = SField::new();
pub const CredentialType: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458783> = SField::new();
pub const FinishFunction: SField<Blob<{ DEFAULT_BLOB_SIZE }>, 458784> = SField::new();
pub const Account: SField<AccountID, 524289> = SField::new();
pub const Owner: SField<AccountID, 524290> = SField::new();
pub const Destination: SField<AccountID, 524291> = SField::new();
pub const Issuer: SField<AccountID, 524292> = SField::new();
pub const Authorize: SField<AccountID, 524293> = SField::new();
pub const Unauthorize: SField<AccountID, 524294> = SField::new();
pub const RegularKey: SField<AccountID, 524296> = SField::new();
pub const NFTokenMinter: SField<AccountID, 524297> = SField::new();
pub const EmitCallback: SField<AccountID, 524298> = SField::new();
pub const Holder: SField<AccountID, 524299> = SField::new();
pub const Delegate: SField<AccountID, 524300> = SField::new();
pub const HookAccount: SField<AccountID, 524304> = SField::new();
pub const OtherChainSource: SField<AccountID, 524306> = SField::new();
pub const OtherChainDestination: SField<AccountID, 524307> = SField::new();
pub const AttestationSignerAccount: SField<AccountID, 524308> = SField::new();
pub const AttestationRewardAccount: SField<AccountID, 524309> = SField::new();
pub const LockingChainDoor: SField<AccountID, 524310> = SField::new();
pub const IssuingChainDoor: SField<AccountID, 524311> = SField::new();
pub const Subject: SField<AccountID, 524312> = SField::new();
pub const Number: SField<u8, 589825> = SField::new();
pub const AssetsAvailable: SField<u8, 589826> = SField::new();
pub const AssetsMaximum: SField<u8, 589827> = SField::new();
pub const AssetsTotal: SField<u8, 589828> = SField::new();
pub const LossUnrealized: SField<u8, 589829> = SField::new();
pub const WasmReturnCode: SField<u8, 655361> = SField::new();
pub const TransactionMetaData: SField<Object, 917506> = SField::new();
pub const CreatedNode: SField<Object, 917507> = SField::new();
pub const DeletedNode: SField<Object, 917508> = SField::new();
pub const ModifiedNode: SField<Object, 917509> = SField::new();
pub const PreviousFields: SField<Object, 917510> = SField::new();
pub const FinalFields: SField<Object, 917511> = SField::new();
pub const NewFields: SField<Object, 917512> = SField::new();
pub const TemplateEntry: SField<Object, 917513> = SField::new();
pub const Memo: SField<Object, 917514> = SField::new();
pub const SignerEntry: SField<Object, 917515> = SField::new();
pub const NFToken: SField<Object, 917516> = SField::new();
pub const EmitDetails: SField<Object, 917517> = SField::new();
pub const Hook: SField<Object, 917518> = SField::new();
pub const Permission: SField<Object, 917519> = SField::new();
pub const Signer: SField<Object, 917520> = SField::new();
pub const Majority: SField<Object, 917522> = SField::new();
pub const DisabledValidator: SField<Object, 917523> = SField::new();
pub const EmittedTxn: SField<Object, 917524> = SField::new();
pub const HookExecution: SField<Object, 917525> = SField::new();
pub const HookDefinition: SField<Object, 917526> = SField::new();
pub const HookParameter: SField<Object, 917527> = SField::new();
pub const HookGrant: SField<Object, 917528> = SField::new();
pub const VoteEntry: SField<Object, 917529> = SField::new();
pub const AuctionSlot: SField<Object, 917530> = SField::new();
pub const AuthAccount: SField<Object, 917531> = SField::new();
pub const XChainClaimProofSig: SField<Object, 917532> = SField::new();
pub const XChainCreateAccountProofSig: SField<Object, 917533> = SField::new();
pub const XChainClaimAttestationCollectionElement: SField<Object, 917534> = SField::new();
pub const XChainCreateAccountAttestationCollectionElement: SField<Object, 917535> = SField::new();
pub const PriceData: SField<Object, 917536> = SField::new();
pub const Credential: SField<Object, 917537> = SField::new();
pub const RawTransaction: SField<Object, 917538> = SField::new();
pub const BatchSigner: SField<Object, 917539> = SField::new();
pub const Book: SField<Object, 917540> = SField::new();
pub const Signers: SField<Array, 983043> = SField::new();
pub const SignerEntries: SField<Array, 983044> = SField::new();
pub const Template: SField<Array, 983045> = SField::new();
pub const Necessary: SField<Array, 983046> = SField::new();
pub const Sufficient: SField<Array, 983047> = SField::new();
pub const AffectedNodes: SField<Array, 983048> = SField::new();
pub const Memos: SField<Array, 983049> = SField::new();
pub const NFTokens: SField<Array, 983050> = SField::new();
pub const Hooks: SField<Array, 983051> = SField::new();
pub const VoteSlots: SField<Array, 983052> = SField::new();
pub const AdditionalBooks: SField<Array, 983053> = SField::new();
pub const Majorities: SField<Array, 983056> = SField::new();
pub const DisabledValidators: SField<Array, 983057> = SField::new();
pub const HookExecutions: SField<Array, 983058> = SField::new();
pub const HookParameters: SField<Array, 983059> = SField::new();
pub const HookGrants: SField<Array, 983060> = SField::new();
pub const XChainClaimAttestations: SField<Array, 983061> = SField::new();
pub const XChainCreateAccountAttestations: SField<Array, 983062> = SField::new();
pub const PriceDataSeries: SField<Array, 983064> = SField::new();
pub const AuthAccounts: SField<Array, 983065> = SField::new();
pub const AuthorizeCredentials: SField<Array, 983066> = SField::new();
pub const UnauthorizeCredentials: SField<Array, 983067> = SField::new();
pub const AcceptedCredentials: SField<Array, 983068> = SField::new();
pub const Permissions: SField<Array, 983069> = SField::new();
pub const RawTransactions: SField<Array, 983070> = SField::new();
pub const BatchSigners: SField<Array, 983071> = SField::new();
pub const CloseResolution: SField<u8, 1048577> = SField::new();
pub const Method: SField<u8, 1048578> = SField::new();
pub const TransactionResult: SField<u8, 1048579> = SField::new();
pub const Scale: SField<u8, 1048580> = SField::new();
pub const AssetScale: SField<u8, 1048581> = SField::new();
pub const TickSize: SField<u8, 1048592> = SField::new();
pub const UNLModifyDisabling: SField<u8, 1048593> = SField::new();
pub const HookResult: SField<u8, 1048594> = SField::new();
pub const WasLockingChainSend: SField<u8, 1048595> = SField::new();
pub const WithdrawalPolicy: SField<u8, 1048596> = SField::new();
pub const TakerPaysCurrency: SField<Hash160, 1114113> = SField::new();
pub const TakerPaysIssuer: SField<Hash160, 1114114> = SField::new();
pub const TakerGetsCurrency: SField<Hash160, 1114115> = SField::new();
pub const TakerGetsIssuer: SField<Hash160, 1114116> = SField::new();
pub const Paths: SField<u8, 1179649> = SField::new();
pub const Indexes: SField<u8, 1245185> = SField::new();
pub const Hashes: SField<u8, 1245186> = SField::new();
pub const Amendments: SField<u8, 1245187> = SField::new();
pub const NFTokenOffers: SField<u8, 1245188> = SField::new();
pub const CredentialIDs: SField<u8, 1245189> = SField::new();
pub const MPTokenIssuanceID: SField<Hash192, 1376257> = SField::new();
pub const ShareMPTID: SField<Hash192, 1376258> = SField::new();
pub const LockingChainIssue: SField<Issue, 1572865> = SField::new();
pub const IssuingChainIssue: SField<Issue, 1572866> = SField::new();
pub const Asset: SField<Issue, 1572867> = SField::new();
pub const Asset2: SField<Issue, 1572868> = SField::new();
pub const XChainBridge: SField<u8, 1638401> = SField::new();
pub const BaseAsset: SField<Currency, 1703937> = SField::new();
pub const QuoteAsset: SField<Currency, 1703938> = SField::new();
pub const Transaction: SField<u8, 655425793> = SField::new();
pub const LedgerEntry: SField<u8, 655491329> = SField::new();
pub const Validation: SField<u8, 655556865> = SField::new();
pub const Metadata: SField<u8, 655622401> = SField::new();
