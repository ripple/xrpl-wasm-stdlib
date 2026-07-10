//! Nested field access: encode a path (sfield codes and array indices) into the compact binary
//! format the host understands, then read a field like `Memos[0].MemoType`.
//!
//! Two APIs share the same buffer layout:
//!
//! - [`TxPathBuilder`] / [`LedgerPathBuilder`] are the recommended fluent builders. They are
//!   rooted at the context —
//!   [`ctx.tx().path()`](crate::fields::current_tx::traits::TransactionCommonFields::path) for the
//!   transaction, [`ctx.escrow().path()`](crate::objects::traits::CurrentLedgerObjectCommonFields::path)
//!   or a slot-bound object's
//!   [`path()`](crate::objects::traits::LedgerObjectCommonFields::path) for a ledger object — so no
//!   bare buffer escapes and the terminal `get` always dispatches to the matching host function.
//!   Field codes come from typed `SField` constants:
//!   ```no_run
//!   use xrpl_common_stdlib::fields::current_tx::traits::TransactionCommonFields;
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

use crate::fields::decoder::{FieldDecoder, FromCurrentTx, FromLedger, finish_field};
use crate::host::error_codes::FIELD_NOT_FOUND;
use crate::host::{
    self, Result, get_current_ledger_obj_nested_field, get_ledger_obj_nested_field,
    get_tx_nested_field,
};
use crate::sfield::SField;

/// The size of the buffer, in bytes, to use for any new locator
const LOCATOR_BUFFER_SIZE: usize = 64; // max depth: 64/4 = 16

/// A Locator encodes a path to a nested field as a sequence of 4-byte packed values
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

/// The retrieval context a [`PathBuf`] reads from: selects which nested-field host function the
/// terminal `get` calls, and for a cached object carries the slot.
#[derive(Clone, PartialEq, Eq, Debug)]
enum NestedSource {
    /// The current transaction (`get_tx_nested_field`).
    Tx,
    /// The ledger object the contract is attached to (`get_current_ledger_obj_nested_field`).
    CurrentLedgerObj,
    /// A ledger object cached in the given slot (`get_ledger_obj_nested_field`).
    LedgerObj(i32),
}

impl NestedSource {
    /// Issue this context's nested-field host call for the packed `locator` bytes.
    fn call(&self, loc_ptr: *const u8, loc_len: usize, out_ptr: *mut u8, out_len: usize) -> i32 {
        match self {
            NestedSource::Tx => unsafe { get_tx_nested_field(loc_ptr, loc_len, out_ptr, out_len) },
            NestedSource::CurrentLedgerObj => unsafe {
                get_current_ledger_obj_nested_field(loc_ptr, loc_len, out_ptr, out_len)
            },
            NestedSource::LedgerObj(slot) => unsafe {
                get_ledger_obj_nested_field(*slot, loc_ptr, loc_len, out_ptr, out_len)
            },
        }
    }
}

/// Shared path-building core behind the public builders: the packed [`Locator`] buffer, the sticky
/// overflow flag, and the [`NestedSource`] to read from. The public wrappers add the context-marker
/// bound (`FromCurrentTx` vs `FromLedger`) at their signatures; `get` / `get_optional` here are
/// bounded on [`FieldDecoder`] because that marker is enforced one level up.
#[derive(Clone, PartialEq, Eq, Debug)]
struct PathBuf {
    locator: Locator,
    /// Set once a `field`/`index` segment did not fit in the buffer. Sticky: further calls stay
    /// overflowed so `get` reports the malformed path instead of reading a truncated one.
    overflowed: bool,
    source: NestedSource,
}

impl PathBuf {
    fn new(source: NestedSource) -> Self {
        Self {
            locator: Locator::new(),
            overflowed: false,
            source,
        }
    }

    /// Append one 4-byte segment, recording buffer overflow so `get` can reject a truncated path.
    fn push(mut self, value: i32) -> Self {
        if !self.locator.pack(value) {
            self.overflowed = true;
        }
        self
    }

    fn get<T: FieldDecoder>(&self) -> Result<T> {
        if self.overflowed {
            return Result::Err(host::Error::LocatorMalformed);
        }
        let (mut buf, n) = self.read::<T>();
        finish_field::<T>(n, &mut buf)
    }

    fn get_optional<T: FieldDecoder>(&self) -> Result<Option<T>> {
        if self.overflowed {
            return Result::Err(host::Error::LocatorMalformed);
        }
        let (mut buf, n) = self.read::<T>();
        if n == FIELD_NOT_FOUND {
            return Result::Ok(None);
        }
        finish_field::<T>(n, &mut buf).map(Some)
    }

    /// Run the built path through the source's host call into a fresh `T` buffer, returning that
    /// buffer and the raw byte count the host reported (negative on error).
    fn read<T: FieldDecoder>(&self) -> (T::Buffer, i32) {
        let mut buf = T::empty_buffer();
        let n = {
            let slice = buf.as_mut();
            self.source.call(
                self.locator.as_ptr(),
                self.locator.num_packed_bytes(),
                slice.as_mut_ptr(),
                slice.len(),
            )
        };
        (buf, n)
    }
}

/// Fluent builder for reading a nested field from the **current transaction**.
///
/// Obtained from the context via
/// [`ctx.tx().path()`](crate::fields::current_tx::traits::TransactionCommonFields::path); rooting
/// it there is what guarantees the terminal [`get`](Self::get) reads through the
/// current-transaction host function and never crosses into a ledger-object read. Each
/// [`field`](Self::field) / [`index`](Self::index) call appends one 4-byte segment to the
/// underlying [`Locator`] buffer.
///
/// A path longer than the 64-byte buffer (more than 16 segments) can hold is not silently
/// truncated: the overflow is remembered and surfaced as [`host::Error::LocatorMalformed`] from
/// [`get`](Self::get), rather than sending the host a shorter path than the author wrote.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TxPathBuilder(PathBuf);

impl TxPathBuilder {
    /// Root a new builder at the current transaction. Callers reach this through
    /// [`TransactionCommonFields::path`](crate::fields::current_tx::traits::TransactionCommonFields::path).
    pub(crate) fn for_current_tx() -> Self {
        Self(PathBuf::new(NestedSource::Tx))
    }

    /// Append a field code to the path.
    ///
    /// Takes a typed [`SField<T, CODE>`] constant (e.g. `sfield::Memos`) so the field code is a
    /// compile-time constant; only the code is encoded — the field's declared type `T` is
    /// irrelevant to the path and is chosen instead at [`get`](Self::get).
    pub fn field<T, const CODE: i32>(self, _field: SField<T, CODE>) -> Self {
        Self(self.0.push(CODE))
    }

    /// Append an array slot index to the path (e.g. the `0` in `Memos[0]`).
    pub fn index(self, index: u32) -> Self {
        // A u32 and its i32 bit-pattern pack to identical bytes, which is what the host reads back.
        Self(self.0.push(index as i32))
    }

    /// Execute `get_tx_nested_field` for the built path and decode the result as `T`.
    ///
    /// `T` picks the terminal type (and therefore the read buffer size and decoder); it must be
    /// readable from a transaction, hence the [`FromCurrentTx`] bound.
    ///
    /// Returns [`host::Error::LocatorMalformed`] without calling the host if the path overflowed
    /// the buffer while being built.
    pub fn get<T: FromCurrentTx>(&self) -> Result<T> {
        self.0.get::<T>()
    }

    /// Like [`get`](Self::get) but treats an absent field as `Ok(None)` rather than an error —
    /// the nested-path counterpart to [`get_field_optional`](crate::fields::current_tx::get_field_optional).
    pub fn get_optional<T: FromCurrentTx>(&self) -> Result<Option<T>> {
        self.0.get_optional::<T>()
    }
}

/// Fluent builder for reading a nested field from a **ledger object** — either the current object
/// the contract is attached to, or one cached in a slot.
///
/// Obtained from the context via
/// [`ctx.escrow().path()`](crate::objects::traits::CurrentLedgerObjectCommonFields::path) or from a
/// slot-bound object handle via
/// [`LedgerObjectCommonFields::path`](crate::objects::traits::LedgerObjectCommonFields::path).
/// Terminal reads are bounded on [`FromLedger`]; behavior otherwise matches [`TxPathBuilder`],
/// including the overflow → [`host::Error::LocatorMalformed`] guard.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LedgerPathBuilder(PathBuf);

impl LedgerPathBuilder {
    /// Root a new builder at the current ledger object (no slot). Callers reach this through
    /// [`CurrentLedgerObjectCommonFields::path`](crate::objects::traits::CurrentLedgerObjectCommonFields::path).
    pub(crate) fn for_current_ledger_obj() -> Self {
        Self(PathBuf::new(NestedSource::CurrentLedgerObj))
    }

    /// Root a new builder at the ledger object cached in `slot`. Callers reach this through
    /// [`LedgerObjectCommonFields::path`](crate::objects::traits::LedgerObjectCommonFields::path).
    pub(crate) fn for_ledger_obj(slot: i32) -> Self {
        Self(PathBuf::new(NestedSource::LedgerObj(slot)))
    }

    /// Append a field code to the path. See [`TxPathBuilder::field`].
    pub fn field<T, const CODE: i32>(self, _field: SField<T, CODE>) -> Self {
        Self(self.0.push(CODE))
    }

    /// Append an array slot index to the path (e.g. the `0` in `Signers[0]`).
    pub fn index(self, index: u32) -> Self {
        Self(self.0.push(index as i32))
    }

    /// Execute the ledger-object nested-field host call for the built path and decode as `T`.
    ///
    /// `T` must be readable from a ledger object, hence the [`FromLedger`] bound. Returns
    /// [`host::Error::LocatorMalformed`] without calling the host if the path overflowed.
    pub fn get<T: FromLedger>(&self) -> Result<T> {
        self.0.get::<T>()
    }

    /// Like [`get`](Self::get) but treats an absent field as `Ok(None)` rather than an error.
    pub fn get_optional<T: FromLedger>(&self) -> Result<Option<T>> {
        self.0.get_optional::<T>()
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

    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use mockall::predicate::{always, eq};

    /// The bytes a `TxPathBuilder` has packed so far, for asserting on the encoded path.
    fn packed(builder: &TxPathBuilder) -> &[u8] {
        &builder.0.locator.buffer[..builder.0.locator.cur_buffer_index]
    }

    #[test]
    fn test_tx_field_encodes_single_field_code() {
        let builder = TxPathBuilder::for_current_tx().field(sfield::Sequence);

        assert!(!builder.0.overflowed);
        assert_eq!(packed(&builder), &i32::from(sfield::Sequence).to_le_bytes());
    }

    #[test]
    fn test_tx_multi_hop_encodes_each_field_in_order() {
        let builder = TxPathBuilder::for_current_tx()
            .field(sfield::Memos)
            .field(sfield::MemoData);

        assert!(!builder.0.overflowed);
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

        assert!(!builder.0.overflowed);
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
        assert!(!builder.0.overflowed);
        assert_eq!(builder.0.locator.num_packed_bytes(), 64);

        let builder = builder.field(sfield::Sequence);
        assert!(builder.0.overflowed);
        // The buffer is not grown or partially overwritten past its capacity.
        assert_eq!(builder.0.locator.num_packed_bytes(), 64);
    }

    #[test]
    fn test_tx_overflow_via_index_sets_flag_and_stops_at_64_bytes() {
        // Same boundary, overflowing with `index` instead of `field`.
        let mut builder = TxPathBuilder::for_current_tx();
        for _ in 0..16 {
            builder = builder.field(sfield::Sequence);
        }
        assert!(!builder.0.overflowed);
        assert_eq!(builder.0.locator.num_packed_bytes(), 64);

        let builder = builder.index(99);
        assert!(builder.0.overflowed);
        assert_eq!(builder.0.locator.num_packed_bytes(), 64);
    }

    #[test]
    fn test_get_reads_and_decodes_nested_field() {
        let mut mock = MockHostBindings::new();
        // Path is Memos[0].MemoData -> three 4-byte segments = 12 bytes; u32 read buffer is 4.
        mock.expect_get_tx_nested_field()
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
        mock.expect_get_tx_nested_field().times(0);
        let _guard = setup_mock(mock);

        let mut builder = TxPathBuilder::for_current_tx();
        for i in 0..17 {
            builder = builder.index(i);
        }
        assert!(builder.0.overflowed);

        let result = builder.get::<u32>();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    #[test]
    fn test_get_propagates_host_error() {
        use crate::host::error_codes::INTERNAL_ERROR;
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_nested_field()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| INTERNAL_ERROR);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::Sequence)
            .get::<u32>();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_get_optional_returns_some_when_present() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_nested_field()
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
        mock.expect_get_tx_nested_field()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = TxPathBuilder::for_current_tx()
            .field(sfield::SourceTag)
            .get_optional::<u32>();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_optional_returns_locator_malformed_when_overflowed() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_tx_nested_field().times(0);
        let _guard = setup_mock(mock);

        let mut builder = TxPathBuilder::for_current_tx();
        for i in 0..17 {
            builder = builder.index(i);
        }

        let result = builder.get_optional::<u32>();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }

    // ---- Ledger-object path builders (`ctx.escrow().path()` / `obj.path()`) ----

    #[test]
    fn test_ledger_current_reads_via_current_obj_host_fn() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_current_ledger_obj_nested_field()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| 4);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::Flags)
            .get::<u32>();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ledger_by_slot_passes_slot_and_reads_via_slot_host_fn() {
        const SLOT: i32 = 7;
        let mut mock = MockHostBindings::new();
        // SignerEntries[0] -> two 4-byte segments = 8 bytes; the slot is threaded through.
        mock.expect_get_ledger_obj_nested_field()
            .with(eq(SLOT), always(), eq(8usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _, _| 4);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_ledger_obj(SLOT)
            .field(sfield::SignerEntries)
            .index(0)
            .get::<u32>();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ledger_get_optional_none_on_field_not_found() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_current_ledger_obj_nested_field()
            .with(always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _| FIELD_NOT_FOUND);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_current_ledger_obj()
            .field(sfield::Flags)
            .get_optional::<u32>();
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_ledger_by_slot_get_optional_some_when_present() {
        const SLOT: i32 = 2;
        let mut mock = MockHostBindings::new();
        mock.expect_get_ledger_obj_nested_field()
            .with(eq(SLOT), always(), eq(4usize), always(), eq(4usize))
            .times(1)
            .returning(|_, _, _, _, _| 4);
        let _guard = setup_mock(mock);

        let result = LedgerPathBuilder::for_ledger_obj(SLOT)
            .field(sfield::Flags)
            .get_optional::<u32>();
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_ledger_overflow_returns_locator_malformed_for_both_terminals() {
        let mut mock = MockHostBindings::new();
        mock.expect_get_ledger_obj_nested_field().times(0);
        let _guard = setup_mock(mock);

        let mut builder = LedgerPathBuilder::for_ledger_obj(1);
        for i in 0..17 {
            builder = builder.index(i);
        }

        assert_eq!(
            builder.get::<u32>().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
        assert_eq!(
            builder.get_optional::<u32>().err().unwrap().code(),
            host::Error::LocatorMalformed.code()
        );
    }
}
