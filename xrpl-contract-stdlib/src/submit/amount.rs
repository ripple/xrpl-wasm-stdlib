//! Transaction-submission helpers for [`Amount`].
//!
//! These live in the contract crate rather than on `Amount` itself: emitting a
//! `Payment` or `TrustSet` is contract behavior, whereas `Amount` (in the base
//! `xrpl-wasm-stdlib` crate) is just a value type. Because inherent methods can
//! only be defined in the crate that owns the type, the behavior is exposed as
//! an extension trait; bring [`AmountSubmit`] into scope to call `.transfer(..)`
//! / `.approve(..)` on an `Amount`.

use crate::core::types::account_id::AccountID;
use crate::core::types::amount::Amount;
use crate::core::types::opaque_float::OpaqueFloat;
use crate::core::types::transaction_type::TransactionType;
use crate::host::{add_txn_field, build_txn, emit_built_txn, float_from_int};
use crate::sfield;

/// Extension trait adding transaction-submission helpers to [`Amount`].
pub trait AmountSubmit {
    /// Emits a `Payment` of this amount to `recipient`. Returns the host emit
    /// result, or a negative error code if building/emitting fails.
    fn transfer(&self, recipient: &AccountID) -> i32;

    /// Emits a `TrustSet` setting the trust-line limit for this IOU's
    /// issuer/currency. `limit_value` is `Some((value, decimals))` to set a
    /// limit, or `None` to zero it out. Only valid for `Amount::IOU`.
    fn approve(&self, limit_value: Option<(i64, i32)>) -> i32;
}

impl AmountSubmit for Amount {
    fn transfer(&self, recipient: &AccountID) -> i32 {
        unsafe {
            // Build Payment transaction
            let txn_index = build_txn(TransactionType::Payment as i32);
            if txn_index < 0 {
                return -100; // Build error
            }

            // Get the encoded amount from Amount
            let (amount_bytes, _) = self.to_stamount_bytes();

            // Add Amount field
            if add_txn_field(
                txn_index,
                sfield::Amount.into(),
                amount_bytes.as_ptr(),
                amount_bytes.len(),
            ) < 0
            {
                return -101; // Field error
            }

            // Add Destination field (21 bytes: 1 byte prefix + 20 byte account)
            let mut dest_buffer = [0u8; 21];
            dest_buffer[0] = 0x14; // Account ID type prefix
            dest_buffer[1..21].copy_from_slice(&recipient.0);

            if add_txn_field(
                txn_index,
                sfield::Destination.into(),
                dest_buffer.as_ptr(),
                dest_buffer.len(),
            ) < 0
            {
                return -102; // Field error
            }

            // Emit the transaction
            emit_built_txn(txn_index)
        }
    }

    fn approve(&self, limit_value: Option<(i64, i32)>) -> i32 {
        match self {
            Amount::IOU {
                issuer, currency, ..
            } => {
                unsafe {
                    // Build TrustSet transaction
                    let txn_index = build_txn(TransactionType::TrustSet as i32);
                    if txn_index < 0 {
                        return -100; // Build error
                    }

                    // Create the limit amount using host function
                    let mut float_bytes = [0u8; 8];

                    let limit_opaque = match limit_value {
                        Some((value, decimals)) => {
                            // Use host function to create OpaqueFloat
                            let result =
                                float_from_int(value, float_bytes.as_mut_ptr(), 8, decimals);

                            if result < 0 {
                                return -104; // Float conversion error
                            }

                            OpaqueFloat(float_bytes)
                        }
                        None => {
                            // Set to zero to remove trust line
                            // Use host function with 0 value
                            let result = float_from_int(0, float_bytes.as_mut_ptr(), 8, 0);

                            if result < 0 {
                                return -104; // Float conversion error
                            }

                            OpaqueFloat(float_bytes)
                        }
                    };

                    // Create the IOU amount with the limit
                    let limit_iou = Amount::IOU {
                        amount: limit_opaque,
                        issuer: *issuer,
                        currency: *currency,
                    };

                    // Get the encoded amount
                    let (amount_bytes, _) = limit_iou.to_stamount_bytes();

                    // Add LimitAmount field
                    if add_txn_field(
                        txn_index,
                        sfield::LimitAmount.into(),
                        amount_bytes.as_ptr(),
                        amount_bytes.len(),
                    ) < 0
                    {
                        return -101; // Field error
                    }

                    // Emit the transaction
                    emit_built_txn(txn_index)
                }
            }
            _ => {
                // TrustSet only works with IOUs
                -103 // Invalid amount type error
            }
        }
    }
}
