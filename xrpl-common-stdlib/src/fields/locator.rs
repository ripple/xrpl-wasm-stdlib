//! Inner field access: encode a path (sfield codes and array indices) into the compact binary
//! format the host understands, then read a field like `Memos[0].MemoType`.
//!
//! Two APIs share the same buffer layout:
//!
//! - [`TxPathBuilder`] and [`LedgerPathBuilder`] are the recommended fluent builders. Each is
//!   rooted at its context —
//!   [`ctx.tx().path()`](crate::current_tx::traits::TransactionCommonFields::path) for the current
//!   transaction, [`obj.path()`](crate::objects::traits::LedgerObjectCommonFields::path) or
//!   [`ctx.escrow().path()`](crate::objects::traits::CurrentLedgerObjectCommonFields::path) for a
//!   ledger object — so no bare buffer escapes and the terminal `get` always dispatches to the
//!   host function matching that context. Field codes come from typed `SField` constants:
//!   ```no_run
//!   use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
//!   use xrpl_common_stdlib::sfield;
//!   # fn demo(tx: &impl TransactionCommonFields) {
//!   // Read Memos[0].MemoData from the current transaction.
//!   let data = tx.path()
//!       .field(sfield::Memos)
//!       .index(0)
//!       .field(sfield::MemoData)
//!       .get::<u32>();
//!   # let _ = data; }
//!   ```
//! - [`Locator`] itself is the lower-level buffer: [`pack`](Locator::pack) values in and pass
//!   [`as_ptr`](Locator::as_ptr) / [`num_packed_bytes`](Locator::num_packed_bytes) to a raw host
//!   call. Prefer the builder unless you need that manual control.
//!   ```no_run
//!   use xrpl_common_stdlib::fields::locator::Locator;
//!   use xrpl_common_stdlib::sfield;
//!   let mut l = Locator::new();
//!   l.pack(sfield::Memos);
//!   l.pack(0);
//!   l.pack(sfield::MemoType);
//!   # let _ = (l.len() >= 3);
//!   ```

use crate::fields::decoder::{FromCurrentTx, FromLedger, decode_host_result};
use crate::host::error_codes::match_result_code;
use crate::host::{
    self, Result, home_le_inner, home_le_inner_arr_len, le_inner, le_inner_arr_len, tx_inner,
    tx_inner_arr_len,
};
use crate::sfield::SField;

/// The size of the buffer, in bytes, to use for any new locator
const LOCATOR_BUFFER_SIZE: usize = 64; // max depth: 64/4 = 16

/// A Locator encodes a path to an inner field as a sequence of 4-byte packed values
/// (sfield codes or array indices) in a compact binary format understood by the host.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Reasonable for this 72-byte struct when explicit copying is needed
/// - `Eq, PartialEq`: Enable comparisons between locators
///
/// Note: `Copy` is intentionally not derived due to the struct's size (72 bytes).
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate a locator.
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Locator {
    buffer: [u8; LOCATOR_BUFFER_SIZE],

    /// An index into `buffer` where the next packing operation can be stored.
    cur_buffer_index: usize,
}

impl Default for Locator {
    fn default() -> Self {
        Self::new()
    }
}

impl Locator {
    /// Create a new empty Locator.
    pub fn new() -> Locator {
        Self {
            buffer: [0; LOCATOR_BUFFER_SIZE],
            cur_buffer_index: 0,
        }
    }

    pub fn pack(&mut self, sfield_or_index: impl Into<i32>) -> bool {
        // Narrow to i32 before the real work so it isn't re-monomorphized per `Into<i32>` caller.
        self.pack_value(sfield_or_index.into())
    }

    fn pack_value(&mut self, value: i32) -> bool {
        if self.cur_buffer_index + 4 > LOCATOR_BUFFER_SIZE {
            return false;
        }

        let value_bytes: [u8; 4] = value.to_le_bytes();
        self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4].copy_from_slice(&value_bytes);
        self.cur_buffer_index += 4;

        true
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn num_packed_bytes(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn len(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn is_empty(&self) -> bool {
        self.cur_buffer_index == 0
    }

    pub fn repack_last(&mut self, sfield_or_index: impl Into<i32>) -> bool {
        self.repack_last_value(sfield_or_index.into())
    }

    fn repack_last_value(&mut self, value: i32) -> bool {
        if self.cur_buffer_index < 4 {
            return false;
        }

        self.cur_buffer_index -= 4;

        let value_bytes: [u8; 4] = value.to_le_bytes();
        self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4].copy_from_slice(&value_bytes);
        self.cur_buffer_index += 4;

        true
    }
}

/// Ask the host how many entries the array a path points at holds, for whichever inner-array-length
/// host call `read` issues.
///
/// Both path builders encode into the same [`Locator`] and differ only in which host function
/// consumes it, so the length protocol — reject a malformed path locally, treat any non-negative
/// answer including zero as a real length — lives here once.
fn array_len_for(
    overflowed: bool,
    locator: &Locator,
    read: impl FnOnce(*const u8, usize) -> i32,
) -> Result<u32> {
    if overflowed {
        return Result::Err(host::Error::LocatorMalformed);
    }
    let n = read(locator.as_ptr(), locator.num_packed_bytes());
    match_result_code(n, || n as u32)
}

/// Fluent builder for reading an inner field from the current transaction.
///
/// Obtained from the context via
/// [`ctx.tx().path()`](crate::current_tx::traits::TransactionCommonFields::path); rooting
/// it there is what guarantees the terminal [`get`](Self::get) reads through the
/// current-transaction host function and never crosses into a ledger-object read. Each
/// [`field`](Self::field) / [`index`](Self::index) call appends one 4-byte segment to the
/// underlying [`Locator`] buffer.
///
/// A path longer than the 64-byte buffer (more than 16 segments) can hold is not silently
/// truncated: the overflow is remembered and surfaced as [`host::Error::LocatorMalformed`] from
/// [`get`](Self::get), rather than sending the host a shorter path than the author wrote.
///
/// ```no_run
/// use xrpl_common_stdlib::current_tx::traits::TransactionCommonFields;
/// use xrpl_common_stdlib::host::Result;
/// use xrpl_common_stdlib::sfield;
/// use xrpl_common_stdlib::types::blob::StandardBlob;
/// # fn demo(tx: &impl TransactionCommonFields) {
/// // Walk every entry of an array field.
/// if let Result::Ok(count) = tx.path().field(sfield::Memos).array_len() {
///     for i in 0..count {
///         let memo_type = tx
///             .path()
///             .field(sfield::Memos)
///             .index(i)
///             .field(sfield::Memo)
///             .field(sfield::MemoType)
///             .get::<StandardBlob>();
///         # let _ = memo_type;
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TxPathBuilder {
    locator: Locator,
    /// Set once a segment could not be encoded — either it did not fit in the buffer, or an
    /// `index` exceeded [`i32::MAX`]. Sticky: further calls stay malformed so the terminals report
    /// the bad path instead of reading a truncated or misencoded one.
    overflowed: bool,
}

impl TxPathBuilder {
    /// Root a new builder at the current transaction. Callers reach this through
    /// [`TransactionCommonFields::path`](crate::current_tx::traits::TransactionCommonFields::path).
    pub(crate) fn for_current_tx() -> Self {
        Self {
            locator: Locator::new(),
            overflowed: false,
        }
    }

    /// Append a field code to the path.
    ///
    /// Takes a typed [`SField<T, CODE>`] constant (e.g. `sfield::Memos`) so the field code is a
    /// compile-time constant; only the code is encoded — the field's declared type `T` is
    /// irrelevant to the path and is chosen instead at [`get`](Self::get).
    pub fn field<T, const CODE: i32>(self, _field: SField<T, CODE>) -> Self {
        self.push(CODE)
    }

    /// Append an array slot index to the path (e.g. the `0` in `Memos[0]`).
    ///
    /// Locator segments are `i32`, so an index above [`i32::MAX`] would pack as a negative value the
    /// host would read back as a field code. Rather than encode that, the path is marked malformed
    /// and the terminals report [`host::Error::LocatorMalformed`]. No array reachable through
    /// [`array_len`](Self::array_len) can be that long — the host reports lengths as `i32` — so this
    /// only rejects an index that did not come from walking the array.
    pub fn index(mut self, index: u32) -> Self {
        match i32::try_from(index) {
            Ok(index) => self.push(index),
            Err(_) => {
                self.overflowed = true;
                self
            }
        }
    }

    /// Append one 4-byte segment, recording buffer overflow so [`get`](Self::get) can reject a
    /// truncated path.
    fn push(mut self, value: i32) -> Self {
        if !self.locator.pack(value) {
            self.overflowed = true;
        }
        self
    }

    /// Execute the `tx_inner` host call for the built path and decode the result as `T`.
    ///
    /// `T` picks the terminal type (and therefore the read buffer size and decoder); it must be
    /// readable from a transaction, hence the [`FromCurrentTx`] bound.
    ///
    /// Returns [`host::Error::LocatorMalformed`] without calling the host if the path overflowed
    /// the buffer while being built.
    pub fn get<T: FromCurrentTx>(&self) -> Result<T> {
        if self.overflowed {
            return Result::Err(host::Error::LocatorMalformed);
        }
        let (buf, n) = self.read::<T>();
        decode_host_result::<T>(buf, n)
    }

    /// Like [`get`](Self::get) but treats an absent field as `Ok(None)` rather than an error —
    /// the inner-path counterpart to
    /// [`get_field_optional`](crate::current_tx::get_field_optional).
    ///
    /// Returns [`host::Error::LocatorMalformed`] without calling the host if the path overflowed.
    pub fn get_optional<T: FromCurrentTx>(&self) -> Result<Option<T>> {
        match self.get::<T>() {
            Result::Ok(value) => Result::Ok(Some(value)),
            Result::Err(host::Error::FieldNotFound) => Result::Ok(None),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Ask the host how many entries the array at this path holds, so it can be iterated.
    ///
    /// Named `array_len` rather than `len` because [`Locator::len`] already means "bytes packed into
    /// the path"; this is the length of the array the path points *at*. Returns `Ok(0)` for a
    /// present-but-empty array, and [`host::Error::LocatorMalformed`] without calling the host if
    /// the path overflowed the buffer while being built.
    ///
    /// A single-segment path — one [`field`](Self::field) and no [`index`](Self::index) — is the
    /// length of a top-level array such as `Memos`, so top-level arrays need no separate accessor.
    pub fn array_len(&self) -> Result<u32> {
        array_len_for(self.overflowed, &self.locator, |loc_ptr, loc_len| unsafe {
            tx_inner_arr_len(loc_ptr, loc_len)
        })
    }

    /// Run the built path through `tx_inner` into a fresh `T` buffer, returning that
    /// buffer and the raw byte count the host reported (negative on error).
    fn read<T: FromCurrentTx>(&self) -> (T::Buffer, i32) {
        let mut buf = T::empty_buffer();
        let n = {
            let slice = buf.as_mut();
            unsafe {
                tx_inner(
                    self.locator.as_ptr(),
                    self.locator.num_packed_bytes(),
                    slice.as_mut_ptr(),
                    slice.len(),
                )
            }
        };
        (buf, n)
    }
}

/// Which ledger object a [`LedgerPath`] reads from: selects the host function the terminals call,
/// and for a slot-cached object carries the slot.
///
/// Deliberately holds only the two *ledger* sources. There is no transaction variant to select, so
/// "a [`LedgerPathBuilder`] cannot read a transaction field" holds by construction rather than by
/// convention — the [`FromLedger`] bound on the public terminals is a second line of defense, not
/// the only one.
#[derive(Clone, PartialEq, Eq, Debug)]
enum LedgerSource {
    /// The ledger object the contract is attached to (`home_le_inner`).
    Current,
    /// A ledger object cached in the given slot (`le_inner`).
    Slot(i32),
}

impl LedgerSource {
    /// Issue this source's inner-field host call for the packed `locator` bytes.
    fn read_field(
        &self,
        loc_ptr: *const u8,
        loc_len: usize,
        out_ptr: *mut u8,
        out_len: usize,
    ) -> i32 {
        match *self {
            LedgerSource::Current => unsafe { home_le_inner(loc_ptr, loc_len, out_ptr, out_len) },
            LedgerSource::Slot(slot) => unsafe {
                le_inner(slot, loc_ptr, loc_len, out_ptr, out_len)
            },
        }
    }

    /// Issue this source's inner-array-length host call for the packed `locator` bytes.
    fn read_array_len(&self, loc_ptr: *const u8, loc_len: usize) -> i32 {
        match *self {
            LedgerSource::Current => unsafe { home_le_inner_arr_len(loc_ptr, loc_len) },
            LedgerSource::Slot(slot) => unsafe { le_inner_arr_len(slot, loc_ptr, loc_len) },
        }
    }
}

/// Fluent builder for reading an inner field from a ledger object — either the object the contract
/// is attached to, or one cached into a slot.
///
/// Obtained from the context via
/// [`obj.path()`](crate::objects::traits::LedgerObjectCommonFields::path) for a slot-cached object
/// or [`ctx.escrow().path()`](crate::objects::traits::CurrentLedgerObjectCommonFields::path) for the
/// current one. Any object type works, including ones with no bespoke wrapper: reach for
/// [`LedgerObject::new(slot)`](crate::objects::LedgerObject::new) to build a handle around a raw slot
/// from [`cache_ledger_entry`](crate::objects::cache_ledger_entry).
///
/// Mirrors [`TxPathBuilder`] — same [`Locator`] buffer, same overflow →
/// [`host::Error::LocatorMalformed`] guard — with two differences: terminal reads are bounded on
/// [`FromLedger`] instead of [`FromCurrentTx`], and a [`LedgerSource`] picks which of the two
/// ledger-object host functions to call.
///
/// ```no_run
/// use xrpl_common_stdlib::host::Result;
/// use xrpl_common_stdlib::objects::traits::LedgerObjectCommonFields;
/// use xrpl_common_stdlib::sfield;
/// # fn demo(obj: &impl LedgerObjectCommonFields) {
/// // Walk every entry of an array field.
/// if let Result::Ok(count) = obj.path().field(sfield::PriceDataSeries).array_len() {
///     for i in 0..count {
///         let price = obj
///             .path()
///             .field(sfield::PriceDataSeries)
///             .index(i)
///             .field(sfield::AssetPrice)
///             .get::<u64>();
///         # let _ = price;
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LedgerPathBuilder {
    locator: Locator,
    /// Set once a segment could not be encoded — either it did not fit in the buffer, or an
    /// `index` exceeded [`i32::MAX`]. Sticky: further calls stay malformed so the terminals report
    /// the bad path instead of reading a truncated or misencoded one.
    overflowed: bool,
    source: LedgerSource,
}

impl LedgerPathBuilder {
    /// Root a new builder at the current ledger object (no slot). Callers reach this through
    /// [`CurrentLedgerObjectCommonFields::path`](crate::objects::traits::CurrentLedgerObjectCommonFields::path).
    pub(crate) fn for_current_ledger_obj() -> Self {
        Self::new(LedgerSource::Current)
    }

    /// Root a new builder at the ledger object cached in `slot`. Callers reach this through
    /// [`LedgerObjectCommonFields::path`](crate::objects::traits::LedgerObjectCommonFields::path).
    pub(crate) fn for_ledger_obj(slot: i32) -> Self {
        Self::new(LedgerSource::Slot(slot))
    }

    fn new(source: LedgerSource) -> Self {
        Self {
            locator: Locator::new(),
            overflowed: false,
            source,
        }
    }

    /// Append a field code to the path. See [`TxPathBuilder::field`].
    pub fn field<T, const CODE: i32>(self, _field: SField<T, CODE>) -> Self {
        self.push(CODE)
    }

    /// Append an array slot index to the path (e.g. the `0` in `SignerEntries[0]`). See
    /// [`TxPathBuilder::index`].
    pub fn index(mut self, index: u32) -> Self {
        match i32::try_from(index) {
            Ok(index) => self.push(index),
            Err(_) => {
                self.overflowed = true;
                self
            }
        }
    }

    /// Append one 4-byte segment, recording overflow so the terminals can reject a truncated path.
    fn push(mut self, value: i32) -> Self {
        if !self.locator.pack(value) {
            self.overflowed = true;
        }
        self
    }

    /// Execute the ledger-object inner-field host call for the built path and decode as `T`.
    ///
    /// `T` must be readable from a ledger object, hence the [`FromLedger`] bound. Returns
    /// [`host::Error::LocatorMalformed`] without calling the host if the path is malformed.
    pub fn get<T: FromLedger>(&self) -> Result<T> {
        if self.overflowed {
            return Result::Err(host::Error::LocatorMalformed);
        }
        let (buf, n) = self.read::<T>();
        decode_host_result::<T>(buf, n)
    }

    /// Like [`get`](Self::get) but treats an absent field as `Ok(None)` rather than an error —
    /// the inner-path counterpart to
    /// [`get_field_optional`](crate::fields::ledger_obj::get_field_optional).
    ///
    /// Only reports absence for fields the host signals with `FieldNotFound`. Variable-length
    /// fields (the `Blob<N>` family) are instead reported as a zero-byte write, so an absent one
    /// yields `Ok(Some(blob))` with `blob.len == 0` rather than `Ok(None)` — the same distinction
    /// [`ledger_obj::get_blob_field_optional`](crate::fields::ledger_obj::get_blob_field_optional)
    /// exists to handle for flat fields.
    pub fn get_optional<T: FromLedger>(&self) -> Result<Option<T>> {
        match self.get::<T>() {
            Result::Ok(value) => Result::Ok(Some(value)),
            Result::Err(host::Error::FieldNotFound) => Result::Ok(None),
            Result::Err(e) => Result::Err(e),
        }
    }

    /// Ask the host how many entries the array at this path holds, so it can be iterated. See
    /// [`TxPathBuilder::array_len`].
    ///
    /// A single-segment path — one [`field`](Self::field) and no [`index`](Self::index) — is the
    /// length of a top-level array such as `SignerEntries`, so top-level arrays need no separate
    /// accessor.
    pub fn array_len(&self) -> Result<u32> {
        array_len_for(self.overflowed, &self.locator, |loc_ptr, loc_len| {
            self.source.read_array_len(loc_ptr, loc_len)
        })
    }

    /// Run the built path through the source's host call into a fresh `T` buffer, returning that
    /// buffer and the raw byte count the host reported (negative on error).
    fn read<T: FromLedger>(&self) -> (T::Buffer, i32) {
        let mut buf = T::empty_buffer();
        let n = {
            let slice = buf.as_mut();
            self.source.read_field(
                self.locator.as_ptr(),
                self.locator.num_packed_bytes(),
                slice.as_mut_ptr(),
                slice.len(),
            )
        };
        (buf, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfield;

    #[test]
    fn test_pack_with_sfield_no_into_needed() {
        // This test demonstrates that .into() is no longer needed when using SField constants
        let mut locator = Locator::new();

        // Pack SField constants directly without .into()
        assert!(locator.pack(sfield::Memos));
        assert!(locator.pack(0));
        assert!(locator.pack(sfield::MemoData));

        assert_eq!(locator.len(), 12); // 3 packed values * 4 bytes each
    }

    #[test]
    fn test_pack_with_i32_still_works() {
        // This test verifies that i32 values still work as before
        let mut locator = Locator::new();

        assert!(locator.pack(123i32));
        assert!(locator.pack(456i32));

        assert_eq!(locator.len(), 8); // 2 packed values * 4 bytes each
    }

    #[test]
    fn test_repack_last_with_sfield() {
        let mut locator = Locator::new();

        locator.pack(sfield::Memos);
        locator.pack(0);

        // Repack the last value with a different SField
        assert!(locator.repack_last(sfield::MemoData));

        assert_eq!(locator.len(), 8); // Still 2 packed values
    }

    #[test]
    fn test_new_starts_empty() {
        let locator = Locator::new();
        assert_eq!(locator.len(), 0);
        assert!(locator.is_empty());
    }

    #[test]
    fn test_default_same_as_new() {
        assert_eq!(Locator::default(), Locator::new());
    }

    #[test]
    fn test_pack_writes_correct_bytes() {
        let mut locator = Locator::new();
        assert!(locator.pack(0x12345678i32));
        assert_eq!(locator.len(), 4);

        let bytes = unsafe { core::slice::from_raw_parts(locator.as_ptr(), 4) };
        assert_eq!(bytes, &0x12345678i32.to_le_bytes());
    }

    #[test]
    fn test_pack_returns_false_when_buffer_full() {
        let mut locator = Locator::new();

        // Fill all 16 slots (64 bytes / 4 bytes per pack)
        for i in 0..16 {
            assert!(locator.pack(i));
        }
        assert_eq!(locator.len(), 64);

        // 17th pack should fail
        assert!(!locator.pack(999i32));
        assert_eq!(locator.len(), 64);
    }

    #[test]
    fn test_is_empty_false_after_pack() {
        let mut locator = Locator::new();
        assert!(locator.is_empty());

        locator.pack(sfield::Memos);
        assert!(!locator.is_empty());
        assert_eq!(locator.len(), 4);
    }

    #[test]
    fn test_num_packed_bytes_equals_len() {
        let mut locator = Locator::new();
        assert_eq!(locator.num_packed_bytes(), locator.len());

        locator.pack(sfield::Memos);
        assert_eq!(locator.num_packed_bytes(), locator.len());
        assert_eq!(locator.num_packed_bytes(), 4);

        locator.pack(0);
        assert_eq!(locator.num_packed_bytes(), locator.len());
        assert_eq!(locator.num_packed_bytes(), 8);
    }

    #[test]
    fn test_repack_last_on_empty_returns_false() {
        let mut locator = Locator::new();
        assert!(!locator.repack_last(sfield::Memos));
        assert_eq!(locator.len(), 0);
    }

    #[test]
    fn test_repack_last_overwrites_correct_bytes() {
        let mut locator = Locator::new();
        locator.pack(0x11111111i32);
        locator.pack(0x22222222i32);
        assert_eq!(locator.len(), 8);

        assert!(locator.repack_last(0x33333333i32));
        assert_eq!(locator.len(), 8);

        let bytes = unsafe { core::slice::from_raw_parts(locator.as_ptr(), 8) };
        // First value unchanged
        assert_eq!(&bytes[0..4], &0x11111111i32.to_le_bytes());
        // Second value replaced
        assert_eq!(&bytes[4..8], &0x33333333i32.to_le_bytes());
    }

    // ---- Fluent path builder (`ctx.tx().path()`) ----

    use crate::host::error_codes::{FIELD_NOT_FOUND, SOME_ERROR};
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::types::blob::StandardBlob;
    use mockall::predicate::{always, eq};

    /// The bytes a `TxPathBuilder` has packed so far, for asserting on the encoded path.
    fn packed(builder: &TxPathBuilder) -> &[u8] {
        &builder.locator.buffer[..builder.locator.cur_buffer_index]
    }

    #[test]
    fn test_tx_field_encodes_single_field_code() {
        let builder = TxPathBuilder::for_current_tx().field(sfield::Sequence);

        assert!(!builder.overflowed);
        assert_eq!(packed(&builder), &i32::from(sfield::Sequence).to_le_bytes());
    }

    #[test]
    fn test_tx_multi_hop_encodes_each_field_in_order() {
        let builder = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .field(sfield::MemoData);

        assert!(!builder.overflowed);
        let bytes = packed(&builder);
        assert_eq!(bytes.len(), 8);
        assert_eq!(&bytes[0..4], &i32::from(sfield::Memos).to_le_bytes());
        assert_eq!(&bytes[4..8], &i32::from(sfield::MemoData).to_le_bytes());
    }

    #[test]
    fn test_tx_index_encodes_array_slot() {
        // Memos[2].MemoType
        let builder = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .index(2)
            .field(sfield::MemoType);

        assert!(!builder.overflowed);
        let bytes = packed(&builder);
        assert_eq!(bytes.len(), 12);
        assert_eq!(&bytes[0..4], &i32::from(sfield::Memos).to_le_bytes());
        assert_eq!(&bytes[4..8], &2u32.to_le_bytes());
        assert_eq!(&bytes[8..12], &i32::from(sfield::MemoType).to_le_bytes());
    }

    #[test]
    fn test_tx_overflow_via_field_sets_flag_and_stops_at_64_bytes() {
        // Fill all 16 slots (64 bytes) with array indices, then one more field can't fit.
        let mut builder = TxPathBuilder::for_current_tx();
        for i in 0..16 {
            builder = builder.index(i);
        }
        assert!(!builder.overflowed);
        assert_eq!(builder.locator.num_packed_bytes(), 64);

        let builder = builder.field(sfield::Sequence);
        assert!(builder.overflowed);
        // The buffer is not grown or partially overwritten past its capacity.
        assert_eq!(builder.locator.num_packed_bytes(), 64);
    }

    #[test]
    fn test_tx_overflow_via_index_sets_flag_and_stops_at_64_bytes() {
        // Same boundary, overflowing with `index` instead of `field`.
        let mut builder = TxPathBuilder::for_current_tx();
        for _ in 0..16 {
            builder = builder.field(sfield::Sequence);
        }
        assert!(!builder.overflowed);
        assert_eq!(builder.locator.num_packed_bytes(), 64);

        let builder = builder.index(99);
        assert!(builder.overflowed);
        assert_eq!(builder.locator.num_packed_bytes(), 64);
    }

    #[test]
    fn test_tx_index_above_i32_max_is_malformed_not_a_negative_segment() {
        // Packing the bit pattern would hand the host a negative segment it reads as a field code.
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner().times(0);
        let _guard = setup_mock(mock);

        let builder = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .index(i32::MAX as u32 + 1);

        assert!(builder.overflowed);
        // The rejected index is not appended, so nothing misencoded reaches the buffer.
        assert_eq!(builder.locator.num_packed_bytes(), 4);
        assert_eq!(
            builder.get::<u32>().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_get_reads_and_decodes_inner_field() {
        let mut mock = MockHostBindings::new();
        // Path is Memos[0].MemoData -> three 4-byte segments = 12 bytes; u32 read buffer is 4.
        mock.expect_tx_inner()
            .with(always(), eq(12usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| 4);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .index(0)
            .field(sfield::MemoData)
            .get::<u32>();

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_returns_locator_malformed_when_overflowed_without_calling_host() {
        // The host must not be queried for a path we know is truncated.
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner().times(0);
        let _guard = setup_mock(mock);

        let mut builder = TxPathBuilder::for_current_tx();
        for i in 0..17 {
            builder = builder.index(i);
        }
        assert!(builder.overflowed);

        let result = builder.get::<u32>();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_get_propagates_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| SOME_ERROR);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Sequence)
            .get::<u32>();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), SOME_ERROR);
    }

    #[test]
    fn test_get_optional_returns_some_when_present() {
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner()
            .with(always(), eq(12usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| 4);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .index(0)
            .field(sfield::MemoData)
            .get_optional::<u32>();

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_get_optional_returns_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Sequence)
            .get_optional::<u32>();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_tx_array_len_returns_count() {
        let mut mock = MockHostBindings::new();
        // A top-level array is a single 4-byte segment of path.
        mock.expect_tx_inner_arr_len()
            .with(always(), eq(4usize))
            .times(1)
            .returning(|_, _| 3);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .array_len();

        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_tx_array_len_zero_is_ok_not_an_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner_arr_len().times(1).returning(|_, _| 0);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .array_len();

        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_tx_array_len_propagates_host_error() {
        use crate::host::error_codes::NO_ARRAY;
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner_arr_len()
            .times(1)
            .returning(|_, _| NO_ARRAY);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Sequence)
            .array_len();

        assert_eq!(result.err().unwrap().code(), NO_ARRAY);
    }

    #[test]
    fn test_tx_array_len_returns_locator_malformed_when_overflowed_without_calling_host() {
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner_arr_len().times(0);
        let _guard = setup_mock(mock);

        let mut builder = TxPathBuilder::for_current_tx();
        for i in 0..17 {
            builder = builder.index(i);
        }

        assert_eq!(
            builder.array_len().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_tx_array_len_then_index_walks_every_entry() {
        // The pattern `array_len()` exists for: count, then read each element.
        let mut mock = MockHostBindings::new();
        mock.expect_tx_inner_arr_len()
            .with(always(), eq(4usize))
            .times(1)
            .returning(|_, _| 2);
        // Two reads of Memos[i].Memo.MemoType -> 16 bytes of path.
        mock.expect_tx_inner()
            .with(always(), eq(16usize), always(), always())
            .times(2)
            .returning(|_, _, _, out_buff_len| out_buff_len as i32);
        let _guard = setup_mock(mock);

        let tx = TxPathBuilder::for_current_tx();
        let count = tx.clone().field(sfield::Memos).array_len().unwrap();
        assert_eq!(count, 2);
        for i in 0..count {
            let memo_type = tx
                .clone()
                .field(sfield::Memos)
                .index(i)
                .field(sfield::Memo)
                .field(sfield::MemoType)
                .get::<StandardBlob>();
            assert!(memo_type.is_ok());
        }
    }

    // ---- Fluent path builder (`obj.path()` / `ctx.escrow().path()`) ----

    /// The bytes a `LedgerPathBuilder` has packed so far, for asserting on the encoded path.
    fn ledger_packed(builder: &LedgerPathBuilder) -> &[u8] {
        &builder.locator.buffer[..builder.locator.cur_buffer_index]
    }

    #[test]
    fn test_ledger_field_encodes_single_field_code() {
        let builder = LedgerPathBuilder::for_current_ledger_obj().field(sfield::Flags);

        assert!(!builder.overflowed);
        assert_eq!(
            ledger_packed(&builder),
            &i32::from(sfield::Flags).to_le_bytes()
        );
    }

    #[test]
    fn test_ledger_index_encodes_array_slot() {
        // SignerEntries[2].Account
        let builder = LedgerPathBuilder::for_ledger_obj(1)
            .field(sfield::SignerEntries)
            .index(2)
            .field(sfield::Account);

        assert!(!builder.overflowed);
        let bytes = ledger_packed(&builder);
        assert_eq!(bytes.len(), 12);
        assert_eq!(
            &bytes[0..4],
            &i32::from(sfield::SignerEntries).to_le_bytes()
        );
        assert_eq!(&bytes[4..8], &2u32.to_le_bytes());
        assert_eq!(&bytes[8..12], &i32::from(sfield::Account).to_le_bytes());
    }

    #[test]
    fn test_ledger_overflow_sets_flag_and_stops_at_64_bytes() {
        // Fill all 16 slots (64 bytes) with array indices, then one more field can't fit.
        let mut builder = LedgerPathBuilder::for_current_ledger_obj();
        for i in 0..16 {
            builder = builder.index(i);
        }
        assert!(!builder.overflowed);
        assert_eq!(builder.locator.num_packed_bytes(), 64);

        let builder = builder.field(sfield::Flags);
        assert!(builder.overflowed);
        // The buffer is not grown or partially overwritten past its capacity.
        assert_eq!(builder.locator.num_packed_bytes(), 64);
    }

    #[test]
    fn test_ledger_index_above_i32_max_is_malformed_not_a_negative_segment() {
        // The counterpart to the transaction-side guard: both builders share the segment encoding.
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner().times(0);
        let _guard = setup_mock(mock);

        let builder = LedgerPathBuilder::for_ledger_obj(1)
            .field(sfield::SignerEntries)
            .index(i32::MAX as u32 + 1);

        assert!(builder.overflowed);
        assert_eq!(builder.locator.num_packed_bytes(), 4);
        assert_eq!(
            builder.get::<u32>().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_ledger_current_get_reads_via_current_obj_host_fn() {
        // A builder rooted at the current object must not reach for the slot-taking host function.
        let mut mock = MockHostBindings::new();
        mock.expect_home_le_inner()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| 4);
        mock.expect_le_inner().times(0);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::Flags)
            .get::<u32>();

        assert!(result.is_ok());
    }

    #[test]
    fn test_ledger_by_slot_get_passes_slot_to_slot_host_fn() {
        const SLOT: i32 = 7;
        let mut mock = MockHostBindings::new();
        // Path is SignerEntries[0] -> two 4-byte segments = 8 bytes; u32 read buffer is 4.
        mock.expect_le_inner()
            .with(eq(SLOT), always(), eq(8usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _, _| 4);
        mock.expect_home_le_inner().times(0);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_ledger_obj(SLOT)
            .field(sfield::SignerEntries)
            .index(0)
            .get::<u32>();

        assert!(result.is_ok());
    }

    #[test]
    fn test_ledger_get_returns_locator_malformed_when_overflowed_without_calling_host() {
        // The host must not be queried for a path we know is truncated.
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner().times(0);
        let _guard = setup_mock(mock);

        let mut builder = LedgerPathBuilder::for_ledger_obj(1);
        for i in 0..17 {
            builder = builder.index(i);
        }
        assert!(builder.overflowed);

        let result = builder.get::<u32>();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_ledger_get_propagates_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_home_le_inner()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| SOME_ERROR);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::Flags)
            .get::<u32>();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), SOME_ERROR);
    }

    #[test]
    fn test_ledger_get_optional_returns_some_when_present() {
        const SLOT: i32 = 2;
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner()
            .with(eq(SLOT), always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _, _| 4);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_ledger_obj(SLOT)
            .field(sfield::Flags)
            .get_optional::<u32>();

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_ledger_get_optional_returns_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_home_le_inner()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::Flags)
            .get_optional::<u32>();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_ledger_index_past_i32_max_marks_path_malformed_without_calling_host() {
        // Packing `u32::MAX as i32` would encode -1, which the host would read back as a field
        // code. The path must be rejected instead of quietly pointing somewhere else.
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner().times(0);
        mock.expect_le_inner_arr_len().times(0);
        let _guard = setup_mock(mock);

        let builder = LedgerPathBuilder::for_ledger_obj(1)
            .field(sfield::SignerEntries)
            .index(u32::MAX);

        assert!(builder.overflowed);
        // The out-of-range segment is not encoded at all, so only `SignerEntries` is packed.
        assert_eq!(builder.locator.num_packed_bytes(), 4);
        assert_eq!(
            builder.get::<u32>().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
        assert_eq!(
            builder.array_len().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    // ---- `array_len()` terminal ----

    #[test]
    fn test_array_len_current_obj_returns_count() {
        let mut mock = MockHostBindings::new();
        mock.expect_home_le_inner_arr_len()
            .with(always(), eq(4usize))
            .times(1)
            .returning(|_, _| 3);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::SignerEntries)
            .array_len();

        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_array_len_by_slot_passes_slot_and_returns_count() {
        const SLOT: i32 = 4;
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner_arr_len()
            .with(eq(SLOT), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _| 2);
        mock.expect_home_le_inner_arr_len().times(0);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_ledger_obj(SLOT)
            .field(sfield::PriceDataSeries)
            .array_len();

        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_array_len_zero_is_ok_not_an_error() {
        // An array that is present but empty is a legitimate answer.
        let mut mock = MockHostBindings::new();
        mock.expect_home_le_inner_arr_len()
            .times(1)
            .returning(|_, _| 0);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::SignerEntries)
            .array_len();

        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_array_len_propagates_host_error() {
        let mut mock = MockHostBindings::new();
        mock.expect_home_le_inner_arr_len()
            .times(1)
            .returning(|_, _| SOME_ERROR);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::SignerEntries)
            .array_len();

        assert_eq!(result.err().unwrap().code(), SOME_ERROR);
    }

    #[test]
    fn test_array_len_returns_locator_malformed_when_overflowed_without_calling_host() {
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner_arr_len().times(0);
        let _guard = setup_mock(mock);

        let mut builder = LedgerPathBuilder::for_ledger_obj(1);
        for i in 0..17 {
            builder = builder.index(i);
        }

        assert_eq!(
            builder.array_len().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_array_len_then_index_walks_every_entry() {
        // The pattern `array_len()` exists for: count, then read each element.
        const SLOT: i32 = 6;
        let mut mock = MockHostBindings::new();
        mock.expect_le_inner_arr_len()
            .with(eq(SLOT), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _| 2);
        // Two reads of PriceDataSeries[i].AssetPrice -> 12 bytes of path, 8-byte u64 buffer.
        mock.expect_le_inner()
            .with(eq(SLOT), always(), eq(12usize), always(), eq(8usize))
            .times(2)
            .returning(|_, _, _, _, _| 8);
        let _guard = setup_mock(mock);

        let obj = LedgerPathBuilder::for_ledger_obj(SLOT);
        let count = obj
            .clone()
            .field(sfield::PriceDataSeries)
            .array_len()
            .unwrap();
        assert_eq!(count, 2);
        for i in 0..count {
            let price = obj
                .clone()
                .field(sfield::PriceDataSeries)
                .index(i)
                .field(sfield::AssetPrice)
                .get::<u64>();
            assert!(price.is_ok());
        }
    }
}
