use crate::core::current_tx::CurrentTxFieldGetter;
use crate::core::ledger_objects::LedgerObjectFieldGetter;
use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::mpt_id::MptId;
use crate::core::types::opaque_float::OpaqueFloat;
use crate::host;
use crate::host::Error::InternalError;
use crate::host::Result::{Err, Ok};
use crate::host::field_helpers::{get_variable_size_field, get_variable_size_field_optional};
use crate::host::trace::trace_num;
use crate::host::{Result, get_current_ledger_obj_field, get_ledger_obj_field, get_tx_field};

pub const AMOUNT_SIZE: usize = 48;

/// A zero-cost abstraction for XRPL tokens. Tokens conform to the following binary layout:
///
/// ```markdown
///              ┌────────────────────────────────────────────────────────────────────────────┐
///              │                       XRP Amount (64 bits / 8 bytes)                       │
///              ├────────────────────────────────────────────────────────────────────────────┤
///              │                     ┌────────────────────────────────────────────────────┐ │
///              │ ┌─┐┌─┐┌─┐ ┌─┬─┬─┬─┐ │ ┌────────────────────────────────────────────────┐ │ │
///              │ │0││1││0│ │0│0│0│0│ │ │                      ...                       │ │ │
///              │ └─┘└─┘└─┘ └─┴─┴─┴─┘ │ └────────────────────────────────────────────────┘ │ │
///              │  ▲  ▲  ▲       ▲    │              Integer Drops (57 bits)               │ │
///              │  │  │  │       │    └────────────────────────────────────────────────────┘ │
///          ┌───┼──┘  │  └─────┐ └────────────────┐                                          │
///          │   └─────┼────────┼──────────────────┼──────────────────────────────────────────┘
///          │         │        │                  │
/// ┌────────────────┐ │ ┌─────────────┐ ┌──────────────────┐
/// │    Type Bit    │ │ │ Is MPT Bit  │ │     Reserved     │
/// │(0=XRP/MPT;1=IOU│ │ │(1=MPT/0=XRP)│ └──────────────────┘
/// └────────────────┘ │ └─────────────┘
///           ┌────────────────┐
///           │    Sign bit    │
///           │(1 for positive)│
///           └────────────────┘
///
///              ┌────────────────────────────────────────────────────────────────────────────┐
///              │                       MPT Amount (264-bits/33-bytes)                       │
///              ├────────────────────────────────────────────────────────────────────────────┤
///              │                       ┌──────────┐ ┌────────────┐ ┌────────────────┐       │
///              │ ┌─┐┌─┐┌─┐ ┌─┬─┬─┬─┬─┐ │┌────────┐│ │ ┌────────┐ │ │   ┌────────┐   │       │
///              │ │0││1││1│ │0│0│0│0│0│ ││  ...   ││ │ │  ...   │ │ │   │  ...   │   │       │
///              │ └─┘└─┘└─┘ └─┴─┴─┴─┴─┘ │└────────┘│ │ └────────┘ │ │   └────────┘   │       │
///              │  ▲  ▲  ▲       ▲      │  Amount  │ │Sequence Num│ │Issuer AccountID│       │
///              │  │  │  │       │      │(64 bits) │ │ (32 bits)  │ │   (160 bits)   │       │
///          ┌───┼──┘  │  └────┐  │      └──────────┘ └────────────┘ └────────────────┘       │
///          │   └─────┼───────┼──┼───────────────────────────────────────────────────────────┘
///          │         │       │  └───────────────┐
/// ┌─────────────────┐│┌─────────────┐           │
/// │    Type Bit     │││ Is MPT Bit  │           │
/// │(0=XRP/MPT;1=IOU)│││(1=MPT/0=XRP)│           │
/// └─────────────────┘│└─────────────┘           │
///           ┌────────────────┐        ┌──────────────────┐
///           │    Sign bit    │        │     Reserved     │
///           │(1 for positive)│        └──────────────────┘
///           └────────────────┘
///
///
///             ┌────────────────────────────────────────────────────────────────────────────────┐
///             │                         IOU Amount (384-bits/48-bytes)                         │
///             ├────────────────────────────────────────────────────────────────────────────────┤
///             │       ┌─────────────────┐  ┌──────────────┐ ┌──────────────┐ ┌────────────────┐│
///             │ ┌─┐┌─┐│┌─┬─┬─┬─┬─┬─┬─┬─┐│  │┌────────────┐│ │  ┌────────┐  │ │   ┌───────┐    ││
///             │ │1││1│││0│0│0│0│0│0│0│0││  ││    ...     ││ │  │  ...   │  │ │   │  ...  │    ││
///             │ └─┘└─┘│└─┴─┴─┴─┴─┴─┴─┴─┘│  │└────────────┘│ │  └────────┘  │ │   └───────┘    ││
///             │  ▲  ▲ │Exponent (8 Bits)│  │Mantissa Bits │ │Currency Code │ │Issuer AccountID││
///             │  │  │ └─────────────────┘  │  (54 Bits)   │ │  (160 bits)  │ │   (160 bits)   ││
///             │  │  └────────────────┐     └──────────────┘ └──────────────┘ └────────────────┘│
///             │  │                   │                                                         │
///             └──┴───────────────────┴─────────────────────────────────────────────────────────┘
///      ┌──────────────────┐┌──────────────────┐
///      │     Type Bit     ││     Sign bit     │
///      │(0=XRP/MPT;1=IOU) ││ (1 for positive) │
///      └──────────────────┘└──────────────────┘
/// ```
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived due to the enum's size (48 bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Amount {
    XRP {
        // amount: Amount::XRP,
        /// Design decision note: Per the pattern in `Amount`, we considered having this be an
        /// unsigned u64 and adding an `is_positve` boolean to this variant. However, we decided to
        /// break that pattern and instead use an i64 here for two reasons. First, this allows
        /// simple math like `add`, `sub`, etc. to be performed in WASM without having to check for
        /// negative values. Second, the total supply of XRP is capped at 100B XRP (100B * 1M Drops),
        /// which fits just fine into an i64.
        num_drops: i64,
    },
    IOU {
        // amount: Amount::IOU,
        amount: OpaqueFloat, // TODO: Make a helper to detect sign from 2nd bit (trait?)
        issuer: AccountID,
        currency: Currency,
    },
    MPT {
        // amount: MptAmount,
        num_units: u64,
        is_positive: bool, // not expected, but just in case.
        mpt_id: MptId,
    },
}

const MASK_57_BIT: u64 = 0x01FFFFFFFFFFFFFFu64;

impl Amount {
    /// Converts a Amount to STAmount bytes format.
    ///
    /// All Amount types return a 48-byte array for consistency with the XRPL STAmount format.
    /// The format follows the XRPL binary layout:
    /// - XRP: Raw drop amount with sign bit in first 8 bytes + 40 bytes padding
    /// - MPT: Flag byte (0b_0110_0000) in byte 0, raw amount in bytes 1-9, MptId in bytes 9-33 + 15 bytes padding
    /// - IOU: OpaqueFloat in first 8 bytes, Currency in bytes 8-28, AccountID in bytes 28-48
    ///
    /// Returns a tuple of (bytes, length) where length is always 48.
    pub fn to_stamount_bytes(&self) -> ([u8; AMOUNT_SIZE], usize) {
        let mut bytes = [0u8; AMOUNT_SIZE];

        match self {
            Amount::XRP { num_drops } => {
                // For tracing, XRP encodes the drop amount with the sign bit
                // Bit 6 is set to 1 for positive amounts, 0 for negative
                let abs_drops = num_drops.unsigned_abs();
                let mut value = abs_drops;
                if *num_drops >= 0 {
                    value |= 0x4000000000000000u64; // Set bit 6 for positive
                }
                bytes[0..8].copy_from_slice(&value.to_be_bytes());
                // Remaining 40 bytes stay as zeros (padding)
            }

            Amount::MPT {
                num_units,
                is_positive,
                mpt_id,
            } => {
                // MPT format for tracing: flag byte + amount + mpt_id
                let mut control_byte = 0u8;

                // Set the sign bit (bit 6)
                if *is_positive {
                    control_byte |= 0x40; // Set bit 6
                }

                // Set the is-MPT bit (bit 5)
                control_byte |= 0x20; // Set bit 5

                // Type bit (bit 7) is 0 for XRP/MPT - already 0
                // Reserved bits (bits 4-0) are 0 - already 0

                bytes[0] = control_byte;
                bytes[1..9].copy_from_slice(&num_units.to_be_bytes());
                bytes[9..33].copy_from_slice(mpt_id.as_bytes());
                // Remaining 15 bytes stay as zeros (padding)
            }

            Amount::IOU {
                amount,
                issuer,
                currency,
            } => {
                // IOU format for tracing: opaque float + currency + issuer
                bytes[0..8].copy_from_slice(&amount.0);
                bytes[8..28].copy_from_slice(currency.as_bytes());
                bytes[28..48].copy_from_slice(&issuer.0);
                // No padding needed - uses all 48 bytes
            }
        }

        (bytes, AMOUNT_SIZE)
    }

    /// Parses a Amount from a byte array.
    ///
    /// The byte array can be one of three formats:
    /// - XRP: 8 bytes
    /// - MPT: 33 bytes
    /// - IOU: 48 bytes
    ///
    /// Returns None if the byte array is not a valid Amount.
    pub fn from_bytes(bytes: &[u8]) -> host::Result<Self> {
        // TODO: Move to trait!

        if bytes.len() != 48 {
            return Err(InternalError);
        }

        let byte0 = bytes[0]; // Get the first byte for flag extraction

        // Extract flags using bitwise operations
        let is_iou = byte0 & 0x80 == 0x80; // Bit 7 (Most Significant Bit)
        let is_xrp_or_mpt = !is_iou;
        let is_xrp: bool = byte0 & 0x20 == 0x00; // Bit 5 (only used if type_bit is 0)

        let is_positive: bool = byte0 & 0x40 == 0x40; // Bit 6

        if is_xrp_or_mpt {
            if is_xrp {
                // If we get here, we'll have 8 bytes.
                let mut amount_bytes = [0u8; 8];
                amount_bytes.copy_from_slice(&bytes[0..8]);

                // For XRP, we need to handle the first byte specially to mask out the flag bits
                // and then use the remaining 7 bytes as is.
                let num_drops_abs = u64::from_be_bytes(amount_bytes) & MASK_57_BIT;

                let amount = Amount::XRP {
                    num_drops: match is_positive {
                        true => num_drops_abs as i64,
                        false => -(num_drops_abs as i64),
                    },
                };

                Ok(amount)
            }
            // is_mpt
            else {
                // If we get here, we'll have 33 bytes.
                // MPT amount: [0/type][1/sign][1/is-mpt][5/reserved][64/value]
                let mut num_units_bytes = [0u8; 8];
                // Skip the first MPT byte, which is control bytes. Grab the next 8 for the u64
                num_units_bytes.copy_from_slice(&bytes[1..9]);
                let num_units = u64::from_be_bytes(num_units_bytes);

                // Parse the MptId from the remaining bytes
                let mut mpt_id_bytes = [0u8; 24];
                mpt_id_bytes.copy_from_slice(&bytes[9..33]);
                let mpt_id = MptId::from(mpt_id_bytes);

                let amount = Amount::MPT {
                    num_units,
                    is_positive,
                    mpt_id,
                };

                Ok(amount)
            }
        }
        // is_iou
        else {
            // If we get here, we'll have 48 bytes.

            // IOU amount: [1/type][1/sign][8/exponent][54/mantissa]
            let opaque_float_amount_bytes: [u8; 8] = bytes[0..8].try_into().unwrap();
            let opaque_float: OpaqueFloat = opaque_float_amount_bytes.into();

            // Parse the Amount::IOU from the first 9 bytes
            // let mut amount_bytes = [0u8; 9];
            // amount_bytes.copy_from_slice(&bytes[0..9]);

            // Parse the Currency from the next 20 bytes
            let mut currency_bytes = [0u8; 20];
            currency_bytes.copy_from_slice(&bytes[8..28]);
            let currency = Currency::from(currency_bytes);

            // Parse the AccountID from the last 20 bytes
            let mut issuer_bytes = [0u8; 20];
            issuer_bytes.copy_from_slice(&bytes[28..48]);
            let issuer = AccountID::from(issuer_bytes);

            let amount = Amount::IOU {
                amount: opaque_float,
                issuer,
                currency,
            };

            Ok(amount)
        }
    }
}

impl From<[u8; AMOUNT_SIZE]> for Amount {
    fn from(bytes: [u8; AMOUNT_SIZE]) -> Self {
        // Use the existing from_bytes method with a slice reference
        match Self::from_bytes(&bytes) {
            Ok(amount) => amount,
            Err(error) => {
                let _ = trace_num("Error parsing amount", error.code() as i64);
                panic!("Invalid Amount byte array");
            }
        }
    }
}

/// Implementation of `LedgerObjectFieldGetter` for XRPL amount values.
///
/// This implementation handles amount fields in XRPL ledger objects, which can represent
/// either XRP amounts (8 bytes) or token amounts (up to 48 bytes including currency code
/// and issuer information).
///
/// # Buffer Management
///
/// Uses a 48-byte buffer to accommodate the largest possible amount representation.
/// The Amount type handles the parsing of different amount formats internally.
/// No strict byte count validation is performed since amounts can vary in size.
impl LedgerObjectFieldGetter for Amount {
    #[inline]
    fn get_from_current_ledger_obj(field_code: i32) -> Result<Self> {
        match get_variable_size_field::<AMOUNT_SIZE, _>(field_code, |fc, buf, size| unsafe {
            get_current_ledger_obj_field(fc, buf, size)
        }) {
            Result::Ok((buffer, _len)) => Result::Ok(Amount::from(buffer)),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_ledger_obj_optional(field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<AMOUNT_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_current_ledger_obj_field(fc, buf, size) },
        ) {
            Result::Ok(opt) => Result::Ok(opt.map(|(buffer, _len)| Amount::from(buffer))),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj(register_num: i32, field_code: i32) -> Result<Self> {
        match get_variable_size_field::<AMOUNT_SIZE, _>(field_code, |fc, buf, size| unsafe {
            get_ledger_obj_field(register_num, fc, buf, size)
        }) {
            Result::Ok((buffer, _len)) => Result::Ok(Amount::from(buffer)),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_ledger_obj_optional(register_num: i32, field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<AMOUNT_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_ledger_obj_field(register_num, fc, buf, size) },
        ) {
            Result::Ok(opt) => Result::Ok(opt.map(|(buffer, _len)| Amount::from(buffer))),
            Result::Err(e) => Result::Err(e),
        }
    }
}

/// Implementation of `CurrentTxFieldGetter` for XRPL amount values.
///
/// This implementation handles amount fields in XRPL transactions, which can represent
/// either XRP amounts (8 bytes) or token amounts (up to 48 bytes including currency code
/// and issuer information). Common uses include transaction fees, payment amounts,
/// offer amounts, and escrow amounts.
///
/// # Buffer Management
///
/// Uses a 48-byte buffer (AMOUNT_SIZE) to accommodate the largest possible amount
/// representation. The Amount type handles the parsing of different amount formats
/// internally. No strict byte count validation is performed since amounts can vary in size.
impl CurrentTxFieldGetter for Amount {
    #[inline]
    fn get_from_current_tx(field_code: i32) -> Result<Self> {
        match get_variable_size_field::<AMOUNT_SIZE, _>(field_code, |fc, buf, size| unsafe {
            get_tx_field(fc, buf, size)
        }) {
            Result::Ok((buffer, _len)) => Result::Ok(Amount::from(buffer)),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn get_from_current_tx_optional(field_code: i32) -> Result<Option<Self>> {
        match get_variable_size_field_optional::<AMOUNT_SIZE, _>(
            field_code,
            |fc, buf, size| unsafe { get_tx_field(fc, buf, size) },
        ) {
            Result::Ok(opt) => Result::Ok(opt.map(|(buffer, _len)| Amount::from(buffer))),
            Result::Err(e) => Result::Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::opaque_float::OpaqueFloat;

    #[test]
    fn test_parse_xrp_amount() {
        // Create a test XRP amount byte array
        // XRP amount: [0/type][1/sign][0/is-mpt][4/reserved][57/value]
        // First byte: 0b0100_0000 (0x40)
        // Value: 1,000,000 (0xF4240 in hex)
        let mut bytes = [0u8; 48];
        bytes[0] = 0x40; // XRP positive flag
        bytes[1..8].copy_from_slice(&1_000_000u64.to_be_bytes()[1..8]);

        // Parse the Amount
        let amount = Amount::from_bytes(&bytes).unwrap();

        // Verify it's an XRP amount with the correct value
        match amount {
            Amount::XRP { num_drops } => {
                assert_eq!(num_drops, 1_000_000);
            }
            _ => panic!("Expected Amount::XRP"),
        }
    }

    #[test]
    fn test_parse_mpt_amount() {
        // Create a test MPT amount byte array
        // MPT amount: [0/type][1/sign][1/is-mpt][5/reserved][64/value][32/sequence][160/issuer]
        // First byte: 0b0110_0000 (0x60)
        const VALUE: u64 = 500_000; // 8 bytes
        const SEQUENCE_NUM: u32 = 12345; // 4 bytes
        const ISSUER_BYTES: [u8; 20] = [1u8; 20]; // 20 bytes

        let mut bytes = [0u8; 48];

        // Set the amount bytes
        bytes[0] = 0x60; // MPT positive flag
        bytes[1..9].copy_from_slice(&VALUE.to_be_bytes());

        // Set the MptId bytes
        bytes[9..13].copy_from_slice(&SEQUENCE_NUM.to_be_bytes());
        // Set the Issuer bytes.
        bytes[13..33].copy_from_slice(&ISSUER_BYTES);

        // Parse the Amount
        let amount = Amount::from_bytes(&bytes).unwrap();

        // Verify it's an MPT amount with the correct values
        match amount {
            Amount::MPT {
                num_units,
                is_positive,
                mpt_id,
            } => {
                assert_eq!(num_units, VALUE);
                assert!(is_positive);
                assert_eq!(mpt_id.get_sequence_num(), SEQUENCE_NUM);
                assert_eq!(mpt_id.get_issuer(), AccountID::from(ISSUER_BYTES));
            }
            _ => panic!("Expected Amount::MPT"),
        }
    }

    #[test]
    fn test_parse_iou_amount() {
        // IOU with exponent = 5, mantissa = 12345
        const EXPONENT: u8 = 5; // 1 byte
        const MANTISSA: u64 = 12345; // 57 bits (so need or 8 bytes)

        // First byte: 0b1100_0000 (0xC0, flags for IOU positive)
        // For exponent 5:
        // - We need to set the last 6 bits of the first byte and first 2 bits of the second byte
        // - 5 = 0b00000101, so we need 0b000001 in the last 6 bits of first byte
        // - and 0b01 in the first 2 bits of second byte

        // Create the input bytes
        let mut input = [0u8; 9];
        // Set the first byte: IOU positive flag (0xC0) with exponent bits
        input[0] = 0xC0 | ((EXPONENT >> 2) & 0x3F); // 5 >> 2 = 1, so this is 0xC1

        // Set the second byte: first 2 bits for exponent, rest will be part of mantissa
        input[1] = (EXPONENT & 0x03) << 6; // 5 & 0x03 = 1, 1 << 6 = 0x40

        let mantissa_bytes = MANTISSA.to_be_bytes();

        // Copy the mantissa bytes to the input array, preserving the exponent bits in input[1]
        // The mantissa starts from the last 6 bits of input[1], then goes for 6 more bytes.
        input[1] |= mantissa_bytes[0] & 0x3F; // Keep first 2 bits for exponent, set last 6 bits from mantissa
        input[2] = mantissa_bytes[1];
        input[3] = mantissa_bytes[2];
        input[4] = mantissa_bytes[3];
        input[5] = mantissa_bytes[4];
        input[6] = mantissa_bytes[5];
        input[7] = mantissa_bytes[6];
        // input[8] = mantissa_bytes[7]; // <-- Not necessary.

        let mut eight_input_bytes: [u8; 8] = [0u8; 8];
        eight_input_bytes.copy_from_slice(&input[..8]);

        /////////////////
        // Add the rest of the Amount Fields
        /////////////////

        // Create a test IOU amount byte array
        // IOU amount: [1/type][1/sign][8/exponent][54/mantissa][160/currency][160/issuer]
        // First byte: 0b1100_0000 (0xC0)

        let mut bytes = [0u8; 48];

        bytes[0..8].copy_from_slice(&eight_input_bytes[0..8]);

        // Set the currency code bytes
        const CURRENCY_BYTES: [u8; 20] = [2u8; 20]; // 20 bytes
        bytes[8..28].copy_from_slice(&CURRENCY_BYTES);

        // Set the issuer bytes
        const ISSUER_BYTES: [u8; 20] = [3u8; 20]; // 20 bytes
        bytes[28..48].copy_from_slice(&ISSUER_BYTES);

        // Parse the Amount
        let amount = Amount::from_bytes(&bytes).unwrap();

        // Verify it's an IOU amount with the correct values
        match amount {
            Amount::IOU {
                amount,
                issuer,
                currency,
            } => {
                assert_eq!(amount, OpaqueFloat(eight_input_bytes));
                assert_eq!(issuer, AccountID::from(ISSUER_BYTES));
                assert_eq!(currency, Currency::from(CURRENCY_BYTES));
            }
            _ => panic!("Expected Amount::IOU"),
        }
    }

    #[test]
    fn test_parse_invalid_amount() {
        // Test with an empty byte array
        assert!(Amount::from_bytes(&[]).is_err());

        // Test with a byte array that's too short for XRP
        assert!(Amount::from_bytes(&[0x40, 0, 0]).is_err());

        // Test with a byte array that's too short for MPT
        let mut mpt_bytes = [0u8; 20];
        mpt_bytes[0] = 0x60; // MPT positive flag
        assert!(Amount::from_bytes(&mpt_bytes).is_err());

        // Test with a byte array that's too short for IOU
        let mut iou_bytes = [0u8; 30];
        iou_bytes[0] = 0xC0; // IOU positive flag
        assert!(Amount::from_bytes(&iou_bytes).is_err());

        // Test with an invalid type bit pattern
        assert!(Amount::from_bytes(&[0xA0, 0, 0, 0, 0, 0, 0, 0]).is_err());
    }

    #[test]
    fn test_round_trip_xrp_positive() {
        // Test positive XRP amount
        let original = Amount::XRP {
            num_drops: 1_000_000,
        };

        // Create the expected byte layout for XRP
        // XRP format: [0/type][1/sign][0/is-mpt][4/reserved][57/value]
        let mut expected_bytes = [0u8; 48];
        expected_bytes[0] = 0x40; // Positive XRP flag (0b0100_0000)
        expected_bytes[1..8].copy_from_slice(&1_000_000u64.to_be_bytes()[1..8]);

        // Test from_bytes -> to_bytes round trip
        let parsed = Amount::from_bytes(&expected_bytes).unwrap();
        assert_eq!(parsed, original);

        // Test to_stamount_bytes format (should include sign bit for positive)
        let (stamount_bytes, len) = original.to_stamount_bytes();
        assert_eq!(len, 48);
        let expected_value = 1_000_000u64 | 0x4000000000000000u64; // Add positive sign bit
        assert_eq!(&stamount_bytes[0..8], &expected_value.to_be_bytes());
        // Remaining bytes should be zero padding
        assert_eq!(&stamount_bytes[8..48], &[0u8; 40]);
    }

    #[test]
    fn test_round_trip_xrp_negative() {
        // Test negative XRP amount
        let original = Amount::XRP {
            num_drops: -500_000,
        };

        // Create the expected byte layout for negative XRP
        // XRP format: [0/type][0/sign][0/is-mpt][4/reserved][57/value]
        let mut expected_bytes = [0u8; 48];
        expected_bytes[0] = 0x00; // Negative XRP flag (0b0000_0000)
        expected_bytes[1..8].copy_from_slice(&500_000u64.to_be_bytes()[1..8]);

        // Test from_bytes -> to_bytes round trip
        let parsed = Amount::from_bytes(&expected_bytes).unwrap();
        assert_eq!(parsed, original);

        // Test to_stamount_bytes format (should NOT include sign bit for negative)
        let (stamount_bytes, len) = original.to_stamount_bytes();
        assert_eq!(len, 48);
        assert_eq!(&stamount_bytes[0..8], &500_000u64.to_be_bytes());
        // Remaining bytes should be zero padding
        assert_eq!(&stamount_bytes[8..48], &[0u8; 40]);
    }

    #[test]
    fn test_round_trip_mpt_positive() {
        // Test positive MPT amount
        const VALUE: u64 = 750_000;
        const SEQUENCE_NUM: u32 = 54321;
        const ISSUER_BYTES: [u8; 20] = [0xAB; 20];

        let issuer = AccountID::from(ISSUER_BYTES);
        let mpt_id = MptId::new(SEQUENCE_NUM, issuer);
        let original = Amount::MPT {
            num_units: VALUE,
            is_positive: true,
            mpt_id,
        };

        // Create the expected byte layout for positive MPT
        // MPT format: [0/type][1/sign][1/is-mpt][5/reserved][64/value][32/sequence][160/issuer]
        let mut expected_bytes = [0u8; 48];
        expected_bytes[0] = 0x60; // Positive MPT flag (0b0110_0000)
        expected_bytes[1..9].copy_from_slice(&VALUE.to_be_bytes());
        expected_bytes[9..13].copy_from_slice(&SEQUENCE_NUM.to_be_bytes());
        expected_bytes[13..33].copy_from_slice(&ISSUER_BYTES);

        // Test from_bytes -> to_bytes round trip
        let parsed = Amount::from_bytes(&expected_bytes).unwrap();
        assert_eq!(parsed, original);

        // Test to_stamount_bytes format
        let (stamount_bytes, len) = original.to_stamount_bytes();
        assert_eq!(len, 48);
        assert_eq!(stamount_bytes[0], 0x60); // Flag byte
        assert_eq!(&stamount_bytes[1..9], &VALUE.to_be_bytes()); // Amount
        assert_eq!(&stamount_bytes[9..33], mpt_id.as_bytes()); // MptId
        // Remaining bytes should be zero padding
        assert_eq!(&stamount_bytes[33..48], &[0u8; 15]);
    }

    #[test]
    fn test_round_trip_mpt_negative() {
        // Test negative MPT amount
        const VALUE: u64 = 250_000;
        const SEQUENCE_NUM: u32 = 98765;
        const ISSUER_BYTES: [u8; 20] = [0xCD; 20];

        let issuer = AccountID::from(ISSUER_BYTES);
        let mpt_id = MptId::new(SEQUENCE_NUM, issuer);
        let original = Amount::MPT {
            num_units: VALUE,
            is_positive: false,
            mpt_id,
        };

        // Create the expected byte layout for negative MPT
        // MPT format: [0/type][0/sign][1/is-mpt][5/reserved][64/value][32/sequence][160/issuer]
        let mut expected_bytes = [0u8; 48];
        expected_bytes[0] = 0x20; // Negative MPT flag (0b0010_0000)
        expected_bytes[1..9].copy_from_slice(&VALUE.to_be_bytes());
        expected_bytes[9..13].copy_from_slice(&SEQUENCE_NUM.to_be_bytes());
        expected_bytes[13..33].copy_from_slice(&ISSUER_BYTES);

        // Test from_bytes -> to_bytes round trip
        let parsed = Amount::from_bytes(&expected_bytes).unwrap();
        assert_eq!(parsed, original);

        // Test to_stamount_bytes format
        let (stamount_bytes, len) = original.to_stamount_bytes();
        assert_eq!(len, 48);
        assert_eq!(stamount_bytes[0], 0x20); // Flag byte (negative)
        assert_eq!(&stamount_bytes[1..9], &VALUE.to_be_bytes()); // Amount
        assert_eq!(&stamount_bytes[9..33], mpt_id.as_bytes()); // MptId
        // Remaining bytes should be zero padding
        assert_eq!(&stamount_bytes[33..48], &[0u8; 15]);
    }

    #[test]
    fn test_round_trip_iou_positive() {
        // Test positive IOU amount
        const EXPONENT: u8 = 7;
        const MANTISSA: u64 = 98765;
        const CURRENCY_BYTES: [u8; 20] = [0xEF; 20];
        const ISSUER_BYTES: [u8; 20] = [0x12; 20];

        // Create the OpaqueFloat bytes manually
        // IOU format: [1/type][1/sign][8/exponent][54/mantissa]
        let mut opaque_float_bytes = [0u8; 8];

        // First byte: IOU positive flag (0xC0) with exponent bits
        opaque_float_bytes[0] = 0xC0 | ((EXPONENT >> 2) & 0x3F);

        // Second byte: first 2 bits for exponent, rest will be part of mantissa
        opaque_float_bytes[1] = (EXPONENT & 0x03) << 6;

        let mantissa_bytes = MANTISSA.to_be_bytes();

        // Copy the mantissa bytes, preserving the exponent bits in opaque_float_bytes[1]
        opaque_float_bytes[1] |= mantissa_bytes[0] & 0x3F;
        opaque_float_bytes[2] = mantissa_bytes[1];
        opaque_float_bytes[3] = mantissa_bytes[2];
        opaque_float_bytes[4] = mantissa_bytes[3];
        opaque_float_bytes[5] = mantissa_bytes[4];
        opaque_float_bytes[6] = mantissa_bytes[5];
        opaque_float_bytes[7] = mantissa_bytes[6];

        let original = Amount::IOU {
            amount: OpaqueFloat(opaque_float_bytes),
            issuer: AccountID::from(ISSUER_BYTES),
            currency: Currency::from(CURRENCY_BYTES),
        };

        // Create the expected byte layout for IOU
        // IOU format: [1/type][1/sign][8/exponent][54/mantissa][160/currency][160/issuer]
        let mut expected_bytes = [0u8; 48];
        expected_bytes[0..8].copy_from_slice(&opaque_float_bytes);
        expected_bytes[8..28].copy_from_slice(&CURRENCY_BYTES);
        expected_bytes[28..48].copy_from_slice(&ISSUER_BYTES);

        // Test from_bytes -> to_bytes round trip
        let parsed = Amount::from_bytes(&expected_bytes).unwrap();
        assert_eq!(parsed, original);

        // Test to_stamount_bytes format
        let (stamount_bytes, len) = original.to_stamount_bytes();
        assert_eq!(len, 48);
        assert_eq!(&stamount_bytes[0..8], &opaque_float_bytes); // OpaqueFloat
        assert_eq!(&stamount_bytes[8..28], &CURRENCY_BYTES); // Currency
        assert_eq!(&stamount_bytes[28..48], &ISSUER_BYTES); // Issuer
        // No padding for IOU - uses all 48 bytes
    }

    #[test]
    fn test_round_trip_iou_negative() {
        // Test negative IOU amount
        const EXPONENT: u8 = 3;
        const MANTISSA: u64 = 12345;
        const CURRENCY_BYTES: [u8; 20] = [0x34; 20];
        const ISSUER_BYTES: [u8; 20] = [0x56; 20];

        // Create the OpaqueFloat bytes manually for negative amount
        // IOU format: [1/type][0/sign][8/exponent][54/mantissa]
        let mut opaque_float_bytes = [0u8; 8];

        // First byte: IOU negative flag (0x80) with exponent bits
        opaque_float_bytes[0] = 0x80 | ((EXPONENT >> 2) & 0x3F);

        // Second byte: first 2 bits for exponent, rest will be part of mantissa
        opaque_float_bytes[1] = (EXPONENT & 0x03) << 6;

        let mantissa_bytes = MANTISSA.to_be_bytes();

        // Copy the mantissa bytes, preserving the exponent bits in opaque_float_bytes[1]
        opaque_float_bytes[1] |= mantissa_bytes[0] & 0x3F;
        opaque_float_bytes[2] = mantissa_bytes[1];
        opaque_float_bytes[3] = mantissa_bytes[2];
        opaque_float_bytes[4] = mantissa_bytes[3];
        opaque_float_bytes[5] = mantissa_bytes[4];
        opaque_float_bytes[6] = mantissa_bytes[5];
        opaque_float_bytes[7] = mantissa_bytes[6];

        let original = Amount::IOU {
            amount: OpaqueFloat(opaque_float_bytes),
            issuer: AccountID::from(ISSUER_BYTES),
            currency: Currency::from(CURRENCY_BYTES),
        };

        // Create the expected byte layout for negative IOU
        // IOU format: [1/type][0/sign][8/exponent][54/mantissa][160/currency][160/issuer]
        let mut expected_bytes = [0u8; 48];
        expected_bytes[0..8].copy_from_slice(&opaque_float_bytes);
        expected_bytes[8..28].copy_from_slice(&CURRENCY_BYTES);
        expected_bytes[28..48].copy_from_slice(&ISSUER_BYTES);

        // Test from_bytes -> to_bytes round trip
        let parsed = Amount::from_bytes(&expected_bytes).unwrap();
        assert_eq!(parsed, original);

        // Test to_stamount_bytes format
        let (stamount_bytes, len) = original.to_stamount_bytes();
        assert_eq!(len, 48);
        assert_eq!(&stamount_bytes[0..8], &opaque_float_bytes); // OpaqueFloat
        assert_eq!(&stamount_bytes[8..28], &CURRENCY_BYTES); // Currency
        assert_eq!(&stamount_bytes[28..48], &ISSUER_BYTES); // Issuer
        // No padding for IOU - uses all 48 bytes
    }

    #[test]
    fn test_round_trip_edge_cases() {
        // Test XRP with maximum value that fits in 57 bits
        let max_57_bit_value = MASK_57_BIT as i64;
        let max_xrp = Amount::XRP {
            num_drops: max_57_bit_value,
        };
        let mut max_xrp_bytes = [0u8; 48];

        // Create the full 64-bit value with flag bits
        let full_value = (max_57_bit_value as u64) | 0x4000000000000000u64; // Add positive flag
        max_xrp_bytes[0..8].copy_from_slice(&full_value.to_be_bytes());

        let parsed_max_xrp = Amount::from_bytes(&max_xrp_bytes).unwrap();
        assert_eq!(parsed_max_xrp, max_xrp);

        // Test XRP with maximum negative value that fits in 57 bits
        let min_xrp = Amount::XRP {
            num_drops: -max_57_bit_value,
        };
        let mut min_xrp_bytes = [0u8; 48];

        // Create the full 64-bit value without positive flag (negative)
        let full_value = max_57_bit_value as u64; // No positive flag = negative
        min_xrp_bytes[0..8].copy_from_slice(&full_value.to_be_bytes());

        let parsed_min_xrp = Amount::from_bytes(&min_xrp_bytes).unwrap();
        assert_eq!(parsed_min_xrp, min_xrp);

        // Test XRP with zero value
        let zero_xrp = Amount::XRP { num_drops: 0 };
        let mut zero_xrp_bytes = [0u8; 48];
        zero_xrp_bytes[0] = 0x40; // Positive flag (zero is considered positive)

        let parsed_zero_xrp = Amount::from_bytes(&zero_xrp_bytes).unwrap();
        assert_eq!(parsed_zero_xrp, zero_xrp);

        // Test that values larger than 57 bits get properly masked during parsing
        let large_value = i64::MAX;
        let expected_masked_value = (large_value as u64 & MASK_57_BIT) as i64;
        let large_xrp = Amount::XRP {
            num_drops: expected_masked_value,
        };

        let mut large_xrp_bytes = [0u8; 48];
        // Create the full 64-bit value with XRP positive flag and the large value
        let masked_value = (large_value as u64) & MASK_57_BIT;
        let full_value = masked_value | 0x4000000000000000u64; // Add positive flag (bit 62)
        large_xrp_bytes[0..8].copy_from_slice(&full_value.to_be_bytes());

        let parsed_large_xrp = Amount::from_bytes(&large_xrp_bytes).unwrap();
        assert_eq!(parsed_large_xrp, large_xrp);
    }
}
