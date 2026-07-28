//! Pure decode-failure signal for [`crate::fields::decoder::FieldDecoder`].

/// Signals that a byte slice could not be decoded into a typed value.
///
/// Implemented so `types/` has no dependency on `host/`. The `fields/`
/// getters (`current_tx::get_field`, `ledger_obj::get_field`, ...) are the place this
/// gets mapped to `host::Error::InvalidDecoding`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeError;
