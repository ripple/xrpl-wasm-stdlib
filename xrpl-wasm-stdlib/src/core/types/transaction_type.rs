use crate::core::current_tx::CurrentTxFieldGetter;
use crate::host::field_helpers::{
    get_fixed_size_field_with_expected_bytes, get_fixed_size_field_with_expected_bytes_optional,
};
use crate::host::{Result, get_tx_field};

/// The type of any given XRPL transaction.
///
/// This enum maps to the transaction type codes used in the XRPL protocol.
/// Each variant corresponds to a specific transaction type with its associated
/// numeric code.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Automatically derived with Copy for consistency
/// - `Copy`: Efficient for this enum (2 bytes due to `#[repr(i16)]`)
/// - `PartialEq, Eq`: Enable transaction type comparisons
///
/// The `Copy` trait is appropriate here because:
/// - The enum is only 2 bytes, making copies extremely cheap
/// - Transaction types are frequently checked and compared
/// - Implicit copying improves ergonomics
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Invalid = -1,
    Payment = 0,
    EscrowCreate = 1,
    EscrowFinish = 2,
    AccountSet = 3,
    EscrowCancel = 4,
    SetRegularKey = 5,
    NickNameSet = 6,
    OfferCreate = 7,
    OfferCancel = 8,
    Contract = 9,
    TicketCreate = 10,
    TicketCancel = 11,
    SignerListSet = 12,
    PaymentChannelCreate = 13,
    PaymentChannelFund = 14,
    PaymentChannelClaim = 15,
    CheckCreate = 16,
    CheckCash = 17,
    CheckCancel = 18,
    DepositPreauth = 19,
    TrustSet = 20,
    AccountDelete = 21,
    SetHook = 22,
    NFTokenMint = 25,
    NFTokenBurn = 26,
    NFTokenCreateOffer = 27,
    NFTokenCancelOffer = 28,
    NFTokenAcceptOffer = 29,
    Clawback = 30,
    AMMCreate = 35,
    AMMDeposit = 36,
    AMMWithdraw = 37,
    AMMVote = 38,
    AMMBid = 39,
    AMMDelete = 40,
    XChainCreateClaimID = 41,
    XChainCommit = 42,
    XChainClaim = 43,
    XChainAccountCreateCommit = 44,
    XChainAddClaimAttestation = 45,
    XChainAddAccountCreateAttestation = 46,
    XChainModifyBridge = 47,
    XChainCreateBridge = 48,
    DIDSet = 49,
    DIDDelete = 50,
    OracleSet = 51,
    OracleDelete = 52,
    EnableAmendment = 100,
    SetFee = 101,
    UNLModify = 102,
}

impl From<[u8; 2]> for TransactionType {
    fn from(value: [u8; 2]) -> Self {
        let value_16 = i16::from_le_bytes(value.as_slice().try_into().expect("Incorrect length"));
        value_16.into()
    }
}

impl From<i16> for TransactionType {
    fn from(value: i16) -> Self {
        match value {
            // List every single variant and its corresponding i16 value
            -1 => TransactionType::Invalid,
            0 => TransactionType::Payment,
            1 => TransactionType::EscrowCreate,
            2 => TransactionType::EscrowFinish,
            3 => TransactionType::AccountSet,
            4 => TransactionType::EscrowCancel,
            5 => TransactionType::SetRegularKey,
            6 => TransactionType::NickNameSet,
            7 => TransactionType::OfferCreate,
            8 => TransactionType::OfferCancel,
            9 => TransactionType::Contract,
            10 => TransactionType::TicketCreate,
            11 => TransactionType::TicketCancel,
            12 => TransactionType::SignerListSet,
            13 => TransactionType::PaymentChannelCreate,
            14 => TransactionType::PaymentChannelFund,
            15 => TransactionType::PaymentChannelClaim,
            16 => TransactionType::CheckCreate,
            17 => TransactionType::CheckCash,
            18 => TransactionType::CheckCancel,
            19 => TransactionType::DepositPreauth,
            20 => TransactionType::TrustSet,
            21 => TransactionType::AccountDelete,
            22 => TransactionType::SetHook,
            25 => TransactionType::NFTokenMint,
            26 => TransactionType::NFTokenBurn,
            27 => TransactionType::NFTokenCreateOffer,
            28 => TransactionType::NFTokenCancelOffer,
            29 => TransactionType::NFTokenAcceptOffer,
            30 => TransactionType::Clawback,
            35 => TransactionType::AMMCreate,
            36 => TransactionType::AMMDeposit,
            37 => TransactionType::AMMWithdraw,
            38 => TransactionType::AMMVote,
            39 => TransactionType::AMMBid,
            40 => TransactionType::AMMDelete,
            41 => TransactionType::XChainCreateClaimID,
            42 => TransactionType::XChainCommit,
            43 => TransactionType::XChainClaim,
            44 => TransactionType::XChainAccountCreateCommit,
            45 => TransactionType::XChainAddClaimAttestation,
            46 => TransactionType::XChainAddAccountCreateAttestation,
            47 => TransactionType::XChainModifyBridge,
            48 => TransactionType::XChainCreateBridge,
            49 => TransactionType::DIDSet,
            50 => TransactionType::DIDDelete,
            51 => TransactionType::OracleSet,
            52 => TransactionType::OracleDelete,
            100 => TransactionType::EnableAmendment,
            101 => TransactionType::SetFee,
            102 => TransactionType::UNLModify,

            // If the value doesn't match any known variant, return an error
            _ => TransactionType::Invalid,
        }
    }
}

impl From<TransactionType> for [u8; 2] {
    fn from(value: TransactionType) -> Self {
        // 1. Cast the enum variant `self` to its underlying i16 value.
        let value_i16: i16 = value as i16;

        // 2. Convert the i16 value into a fixed-size byte array ([u8; 2]).
        let bytes_array: [u8; 2] = value_i16.to_le_bytes();

        bytes_array
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL TransactionType enums.
///
/// This implementation handles 2-byte transaction type fields in XRPL transactions.
///
/// # Buffer Management
///
/// Uses a 2-byte buffer and validates that exactly 2 bytes are returned from the host function.
impl CurrentTxFieldGetter for TransactionType {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        get_fixed_size_field_with_expected_bytes::<2, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        })
        .map(|buffer| i16::from_le_bytes(buffer).into())
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        get_fixed_size_field_with_expected_bytes_optional::<2, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        )
        .map(|buffer| buffer.map(|b| i16::from_le_bytes(b).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type_from_i16_valid() {
        // Test all valid transaction types - documents the i16 -> enum mapping
        assert_eq!(TransactionType::from(-1i16), TransactionType::Invalid);
        assert_eq!(TransactionType::from(0i16), TransactionType::Payment);
        assert_eq!(TransactionType::from(1i16), TransactionType::EscrowCreate);
        assert_eq!(TransactionType::from(2i16), TransactionType::EscrowFinish);
        assert_eq!(TransactionType::from(3i16), TransactionType::AccountSet);
        assert_eq!(TransactionType::from(4i16), TransactionType::EscrowCancel);
        assert_eq!(TransactionType::from(5i16), TransactionType::SetRegularKey);
        assert_eq!(TransactionType::from(6i16), TransactionType::NickNameSet);
        assert_eq!(TransactionType::from(7i16), TransactionType::OfferCreate);
        assert_eq!(TransactionType::from(8i16), TransactionType::OfferCancel);
        assert_eq!(TransactionType::from(9i16), TransactionType::Contract);
        assert_eq!(TransactionType::from(10i16), TransactionType::TicketCreate);
        assert_eq!(TransactionType::from(11i16), TransactionType::TicketCancel);
        assert_eq!(TransactionType::from(12i16), TransactionType::SignerListSet);
        assert_eq!(
            TransactionType::from(13i16),
            TransactionType::PaymentChannelCreate
        );
        assert_eq!(
            TransactionType::from(14i16),
            TransactionType::PaymentChannelFund
        );
        assert_eq!(
            TransactionType::from(15i16),
            TransactionType::PaymentChannelClaim
        );
        assert_eq!(TransactionType::from(16i16), TransactionType::CheckCreate);
        assert_eq!(TransactionType::from(17i16), TransactionType::CheckCash);
        assert_eq!(TransactionType::from(18i16), TransactionType::CheckCancel);
        assert_eq!(
            TransactionType::from(19i16),
            TransactionType::DepositPreauth
        );
        assert_eq!(TransactionType::from(20i16), TransactionType::TrustSet);
        assert_eq!(TransactionType::from(21i16), TransactionType::AccountDelete);
        assert_eq!(TransactionType::from(22i16), TransactionType::SetHook);
        assert_eq!(TransactionType::from(25i16), TransactionType::NFTokenMint);
        assert_eq!(TransactionType::from(26i16), TransactionType::NFTokenBurn);
        assert_eq!(
            TransactionType::from(27i16),
            TransactionType::NFTokenCreateOffer
        );
        assert_eq!(
            TransactionType::from(28i16),
            TransactionType::NFTokenCancelOffer
        );
        assert_eq!(
            TransactionType::from(29i16),
            TransactionType::NFTokenAcceptOffer
        );
        assert_eq!(TransactionType::from(30i16), TransactionType::Clawback);
        assert_eq!(TransactionType::from(35i16), TransactionType::AMMCreate);
        assert_eq!(TransactionType::from(36i16), TransactionType::AMMDeposit);
        assert_eq!(TransactionType::from(37i16), TransactionType::AMMWithdraw);
        assert_eq!(TransactionType::from(38i16), TransactionType::AMMVote);
        assert_eq!(TransactionType::from(39i16), TransactionType::AMMBid);
        assert_eq!(TransactionType::from(40i16), TransactionType::AMMDelete);
        assert_eq!(
            TransactionType::from(41i16),
            TransactionType::XChainCreateClaimID
        );
        assert_eq!(TransactionType::from(42i16), TransactionType::XChainCommit);
        assert_eq!(TransactionType::from(43i16), TransactionType::XChainClaim);
        assert_eq!(
            TransactionType::from(44i16),
            TransactionType::XChainAccountCreateCommit
        );
        assert_eq!(
            TransactionType::from(45i16),
            TransactionType::XChainAddClaimAttestation
        );
        assert_eq!(
            TransactionType::from(46i16),
            TransactionType::XChainAddAccountCreateAttestation
        );
        assert_eq!(
            TransactionType::from(47i16),
            TransactionType::XChainModifyBridge
        );
        assert_eq!(
            TransactionType::from(48i16),
            TransactionType::XChainCreateBridge
        );
        assert_eq!(TransactionType::from(49i16), TransactionType::DIDSet);
        assert_eq!(TransactionType::from(50i16), TransactionType::DIDDelete);
        assert_eq!(TransactionType::from(51i16), TransactionType::OracleSet);
        assert_eq!(TransactionType::from(52i16), TransactionType::OracleDelete);
        assert_eq!(
            TransactionType::from(100i16),
            TransactionType::EnableAmendment
        );
        assert_eq!(TransactionType::from(101i16), TransactionType::SetFee);
        assert_eq!(TransactionType::from(102i16), TransactionType::UNLModify);
    }

    #[test]
    fn test_transaction_type_from_i16_invalid() {
        // Test that gaps in the enum and out-of-range values return Invalid
        assert_eq!(TransactionType::from(999i16), TransactionType::Invalid);
        assert_eq!(TransactionType::from(-100i16), TransactionType::Invalid);
        assert_eq!(TransactionType::from(23i16), TransactionType::Invalid); // Gap in enum
        assert_eq!(TransactionType::from(24i16), TransactionType::Invalid); // Gap in enum
        assert_eq!(TransactionType::from(31i16), TransactionType::Invalid); // Gap in enum
    }

    #[test]
    fn test_transaction_type_to_bytes() {
        // Test serialization to little-endian bytes
        let bytes: [u8; 2] = TransactionType::Payment.into();
        assert_eq!(bytes, [0, 0]); // 0 in little-endian

        let bytes: [u8; 2] = TransactionType::EscrowFinish.into();
        assert_eq!(bytes, [2, 0]); // 2 in little-endian

        let bytes: [u8; 2] = TransactionType::Invalid.into();
        assert_eq!(bytes, [0xff, 0xff]); // -1 in little-endian (two's complement)

        let bytes: [u8; 2] = TransactionType::EnableAmendment.into();
        assert_eq!(bytes, [100, 0]); // 100 in little-endian
    }

    #[test]
    fn test_transaction_type_from_bytes() {
        // Test deserialization from little-endian bytes
        let tx_type = TransactionType::from([0, 0]);
        assert_eq!(tx_type, TransactionType::Payment);

        let tx_type = TransactionType::from([2, 0]);
        assert_eq!(tx_type, TransactionType::EscrowFinish);

        let tx_type = TransactionType::from([0xff, 0xff]);
        assert_eq!(tx_type, TransactionType::Invalid);

        let tx_type = TransactionType::from([100, 0]);
        assert_eq!(tx_type, TransactionType::EnableAmendment);
    }

    #[test]
    fn test_transaction_type_bytes_roundtrip() {
        // Test that converting to bytes and back gives the same result
        let types = [
            TransactionType::Payment,
            TransactionType::EscrowCreate,
            TransactionType::EscrowFinish,
            TransactionType::AccountSet,
            TransactionType::TrustSet,
            TransactionType::AMMCreate,
            TransactionType::NFTokenMint,
            TransactionType::Clawback,
            TransactionType::EnableAmendment,
        ];

        for tx_type in types {
            let bytes: [u8; 2] = tx_type.into();
            let recovered = TransactionType::from(bytes);
            assert_eq!(tx_type, recovered);
        }
    }
}
