use crate::host;
use crate::host::Result;
use crate::host::error_codes::match_result_code_with_expected_bytes;
use crate::types::account_id::AccountID;
use crate::types::currency::Currency;
use crate::types::issue::Issue;
use crate::types::mpt_id::MptId;

pub const XRPL_LEDGER_ENTRY_ID_SIZE: usize = 32;
// Type aliases for specific ledger entry IDs, all currently using the same underlying array type.
pub type LedgerEntryIdBytes = [u8; XRPL_LEDGER_ENTRY_ID_SIZE];

/// Generates an account ledger entry ID for a given XRP Ledger account.
///
/// Account ledger entry IDs are used to reference account entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte account ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::accountroot_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::accountroot_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match accountroot_id(&account){
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn accountroot_id(account_id: &AccountID) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::accountroot_id(
            account_id.0.as_ptr(), // Assuming AccountID is a tuple struct like AccountID(bytes)
            account_id.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an AMM ledger entry ID for a given pair of accounts and currency code.
///
/// An AMM ledger entry ID is used to reference AMM entries in the XRP Ledger.
///
/// # Arguments
///
/// * `issue1` - The first Issue in the AMM relationship
/// * `issue2` - The second Issue in the AMM relationship
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte AMM ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::amm_id` function.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::types::issue::{Issue, XrpIssue, IouIssue};
/// use xrpl_common_stdlib::types::currency::Currency;
/// use xrpl_common_stdlib::ledger_entry_ids::amm_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let issue1: Issue = Issue::XRP(XrpIssue {});
///  let issuer: AccountID =
///    AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///  let currency = b"RLUSD\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"; // RLUSD currency code
///  let currency: Currency = Currency::from(*currency);
///  let issue2 = Issue::IOU(IouIssue::new(issuer, currency));
///  match amm_id(&issue1, &issue2) {
///    xrpl_common_stdlib::host::Result::Ok(id) => {
///      let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///    }
///    xrpl_common_stdlib::host::Result::Err(e) => {
///      let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///    }
///  }
///  Ok(())
/// }
/// ```
pub fn amm_id(issue1: &Issue, issue2: &Issue) -> Result<LedgerEntryIdBytes> {
    let issue1_bytes = issue1.as_bytes();
    let issue2_bytes = issue2.as_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::amm_id(
            issue1_bytes.as_ptr(),
            issue1_bytes.len(),
            issue2_bytes.as_ptr(),
            issue2_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an check ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Check ledger entry IDs are used to reference check entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the check owner's account
/// * `seq` - The account sequence associated with the check entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte check ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::check_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::check_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match check_id(&owner, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn check_id(owner: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::check_id(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a credential ledger entry ID for a given subject, issuer, and credential type.
///
/// A credential ledger entry ID is used to reference credential entries in the XRP Ledger.
///
/// # Arguments
///
/// * `subject` - The AccountID of the subject for whom the credential is issued
/// * `issuer` - The AccountID of the entity issuing the credential
/// * `credential_type` - A byte slice representing the type of credential
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte credential ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::credential_id` function.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::credential_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let subject: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let issuer: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let cred_type: &[u8] = b"termsandconditions";
///     match credential_id(&subject, &issuer, cred_type) {
///       xrpl_common_stdlib::host::Result::Ok(id) => {
///         let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///       }
///       xrpl_common_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn credential_id(
    subject: &AccountID,
    issuer: &AccountID,
    credential_type: &[u8],
) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::credential_id(
            subject.0.as_ptr(),
            subject.0.len(),
            issuer.0.as_ptr(),
            issuer.0.len(),
            credential_type.as_ptr(),
            credential_type.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a delegate ledger entry ID for a given given account and authorized account.
///
/// A delegate ledger entry ID is used to reference delegate entries in the XRP Ledger.
///
/// # Arguments
///
/// * `account` - The AccountID of the account that is delegating permissions
/// * `authorize` - The AccountID of the account that is delegated to
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte delegate ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::delegate_id` function.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::delegate_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let account: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let authorize: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     match delegate_id(&account, &authorize) {
///       xrpl_common_stdlib::host::Result::Ok(id) => {
///         let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///       }
///       xrpl_common_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn delegate_id(account: &AccountID, authorize: &AccountID) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::delegate_id(
            account.0.as_ptr(),
            account.0.len(),
            authorize.0.as_ptr(),
            authorize.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a deposit preauth ledger entry ID for a given account and authorized account.
///
/// A deposit preauth ledger entry ID is used to reference deposit preauth entries in the XRP Ledger.
///
/// # Arguments
///
/// * `account` - The AccountID of the account that is doing the pre-authorizing
/// * `authorize` - The AccountID of the account that is pre-authorizing
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte deposit preauth ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::deposit_preauth_id` function.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::deposit_preauth_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let account: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let authorize: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     match deposit_preauth_id(&account, &authorize) {
///       xrpl_common_stdlib::host::Result::Ok(id) => {
///         let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///       }
///       xrpl_common_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn deposit_preauth_id(
    account: &AccountID,
    authorize: &AccountID,
) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::deposit_preauth_id(
            account.0.as_ptr(),
            account.0.len(),
            authorize.0.as_ptr(),
            authorize.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a DID ledger entry ID for a given XRP Ledger account.
///
/// DID ledger entry IDs are used to reference DID entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte DID ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::did_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::did_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match did_id(&account){
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn did_id(account_id: &AccountID) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::did_id(
            account_id.0.as_ptr(),
            account_id.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an escrow ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Escrow ledger entry IDs are used to reference escrow entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the escrow owner's account
/// * `seq` - The account sequence associated with the escrow entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte escrow ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::escrow_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::escrow_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match escrow_id(&owner, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn escrow_id(owner: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::escrow_id(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a trustline ledger entry ID for a given pair of accounts and currency code.
///
/// A trustline ledger entry ID is used to reference trustline entries in the XRP Ledger.
///
/// # Arguments
///
/// * `account` - The first AccountID in the trustline relationship
/// * `account2` - The second AccountID in the trustline relationship
/// * `currency` - The Currency for the trustline
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte trustline ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::trustline_id` function.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::types::currency::Currency;
/// use xrpl_common_stdlib::ledger_entry_ids::trustline_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let account1: AccountID =
///    AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///  let account2: AccountID =
///    AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///  let currency = b"RLUSD\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"; // RLUSD currency code
///  let currency: Currency = Currency::from(*currency);
///  match trustline_id(&account1, &account2, &currency) {
///    xrpl_common_stdlib::host::Result::Ok(id) => {
///      let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///    }
///    xrpl_common_stdlib::host::Result::Err(e) => {
///      let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///    }
///  }
///  Ok(())
/// }
/// ```
pub fn trustline_id(
    account1: &AccountID,
    account2: &AccountID,
    currency: &Currency,
) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::trustline_id(
            account1.0.as_ptr(),
            account1.0.len(),
            account2.0.as_ptr(),
            account2.0.len(),
            currency.0.as_ptr(),
            currency.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an MPT issuance ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// MPT issuance ledger entry IDs are used to reference MPT issuance entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the MPT issuer's account
/// * `seq` - The account sequence associated with the MPT issuance entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte MPT issuance ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::mpt_issuance_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::mpt_issuance_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match mpt_issuance_id(&owner, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn mpt_issuance_id(owner: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::mpt_issuance_id(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an MPToken ledger entry ID for a given MPT ID and holder.
///
/// An MPToken ledger entry ID is used to reference MPToken entries in the XRP Ledger.
///
/// # Arguments
///
/// * `mptid` - The MPT ID that the MPToken is associated with
/// * `holder` - The AccountID of the account that holds the MPToken
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte MPToken ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::mptoken_id` function.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::types::mpt_id::MptId;
/// use xrpl_common_stdlib::ledger_entry_ids::mptoken_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let issuer: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let mptid: MptId = MptId::new(1, issuer);
///     let holder: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     match mptoken_id(&mptid, &holder) {
///       xrpl_common_stdlib::host::Result::Ok(id) => {
///         let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///       }
///       xrpl_common_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn mptoken_id(mptid: &MptId, holder: &AccountID) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::mptoken_id(
            mptid.as_bytes().as_ptr(),
            mptid.as_bytes().len(),
            holder.0.as_ptr(),
            holder.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an NFT offer ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// NFT offer ledger entry IDs are used to reference NFT offer entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the NFT offer owner's account
/// * `seq` - The account sequence associated with the NFT offer entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte NFT offer ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::nft_offer_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::nft_offer_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match nft_offer_id(&owner, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn nft_offer_id(owner: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::nft_offer_id(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an offer ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Offer ledger entry IDs are used to reference offer entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the offer owner's account
/// * `seq` - The account sequence associated with the offer entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte offer ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::offer_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::offer_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match offer_id(&owner, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn offer_id(owner: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::offer_id(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates an oracle ledger entry ID for a given owner and document ID in the XRP Ledger.
///
/// Oracle ledger entry IDs are used to reference oracle entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the oracle owner's account
/// * `document_id` - An integer identifier for the oracle document
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte oracle ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::oracle_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::oracle_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let document_id = 12345;
///   match oracle_id(&owner, document_id) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn oracle_id(owner: &AccountID, document_id: u32) -> Result<LedgerEntryIdBytes> {
    let document_id_bytes = document_id.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::oracle_id(
            owner.0.as_ptr(),
            owner.0.len(),
            document_id_bytes.as_ptr(),
            document_id_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a payment channel ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Payment channel ledger entry IDs are used to reference payment channel entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account` - Reference to an `AccountID` representing the payment channel sender's account
/// * `destination` - Reference to an `AccountID` representing the payment channel's destination
/// * `seq` - The account sequence associated with the payment channel entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte payment channel ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::paychan_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::paychan_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let destination: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match paychan_id(&account, &destination, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn paychan_id(
    account: &AccountID,
    destination: &AccountID,
    seq: u32,
) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::paychan_id(
            account.0.as_ptr(),
            account.0.len(),
            destination.0.as_ptr(),
            destination.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a permissioned domain ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Permissioned domain ledger entry IDs are used to reference permissioned domain entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account` - Reference to an `AccountID` representing the permissioned domain's owner
/// * `seq` - The account sequence associated with the permissioned domain entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte permissioned domain ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::permissioned_domain_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::permissioned_domain_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match permissioned_domain_id(&account, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn permissioned_domain_id(account: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::permissioned_domain_id(
            account.0.as_ptr(),
            account.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a signer entry ledger entry ID for a given XRP Ledger account.
///
/// signer entry ledger entry IDs are used to reference signer entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte signer entry ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::signers_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::signers_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match signers_id(&account){
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn signers_id(account_id: &AccountID) -> Result<LedgerEntryIdBytes> {
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::signers_id(
            account_id.0.as_ptr(),
            account_id.0.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a ticket ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Ticket ledger entry IDs are used to reference ticket entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the ticket owner's account
/// * `seq` - The account sequence associated with the ticket entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte ticket ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::ticket_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::ticket_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match ticket_id(&owner, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn ticket_id(owner: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::ticket_id(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generates a vault ledger entry ID for a given owner and sequence in the XRP Ledger.
///
/// Vault ledger entry IDs are used to reference vault entries in the XRP Ledger's state data.
/// This function uses the generic `create_id_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account` - Reference to an `AccountID` representing the vault's owner
/// * `seq` - The account sequence associated with the vault entry
///
/// # Returns
///
/// * `Result<LedgerEntryIdBytes>` - On success, returns a 32-byte vault ledger entry ID.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::vault_id` function, though the unsafe code is contained
/// within the closure passed to `create_id_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_common_stdlib::types::account_id::AccountID;
/// use xrpl_common_stdlib::ledger_entry_ids::vault_id;
/// use xrpl_common_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match vault_id(&account, sequence) {
///     xrpl_common_stdlib::host::Result::Ok(id) => {
///       let _ = trace_data("Generated ledger entry ID", &id, DataRepr::AsHex);
///     }
///     xrpl_common_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling ledger entry ID", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn vault_id(account: &AccountID, seq: u32) -> Result<LedgerEntryIdBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_id_from_host_call(|id_buffer_ptr, id_buffer_len| unsafe {
        host::vault_id(
            account.0.as_ptr(),
            account.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            id_buffer_ptr,
            id_buffer_len,
        )
    })
}

/// Generic helper function to create a ledger entry ID by calling a host function.
///
/// This function handles the common tasks of:
/// - Initializing the ledger entry ID output buffer.
/// - Invoking the provided `host_call` closure (which performs the unsafe host FFI call).
/// - Converting the host call's `i32` result code into a `Result<LedgerEntryIdBytes, Error>`.
///
/// # Arguments
///
/// * `host_call`: A closure that takes a mutable pointer to the output buffer (`*mut u8`)
///   and its length (`usize`), performs the specific host FFI call, and returns an `i32` status
///   code.
fn create_id_from_host_call<F>(host_call: F) -> Result<LedgerEntryIdBytes>
where
    F: FnOnce(*mut u8, usize) -> i32,
{
    let mut id_buffer: LedgerEntryIdBytes = [0; XRPL_LEDGER_ENTRY_ID_SIZE];
    let result_code: i32 = host_call(id_buffer.as_mut_ptr(), id_buffer.len());

    match_result_code_with_expected_bytes(result_code, XRPL_LEDGER_ENTRY_ID_SIZE, || id_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    const EXPECTED_ID: LedgerEntryIdBytes = [0xCC; XRPL_LEDGER_ENTRY_ID_SIZE];

    /// Writes `0xCC` into the output buffer and returns `XRPL_LEDGER_ENTRY_ID_SIZE` as success.
    fn write_id_to_buffer(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        assert_eq!(out_buff_len, XRPL_LEDGER_ENTRY_ID_SIZE);
        unsafe {
            for i in 0..XRPL_LEDGER_ENTRY_ID_SIZE {
                *out_buff_ptr.add(i) = 0xCC;
            }
        }
        XRPL_LEDGER_ENTRY_ID_SIZE as i32
    }

    /// Generates a mock `returning` closure that delegates to `write_id_to_buffer`.
    /// Pass the number of prefix parameters (before the out_buff_ptr/out_buff_len pair)
    /// to match the host function arity.
    macro_rules! write_id_returning {
        (2) => {
            |_, _, out_buff_ptr, out_buff_len| write_id_to_buffer(out_buff_ptr, out_buff_len)
        };
        (4) => {
            |_, _, _, _, out_buff_ptr, out_buff_len| write_id_to_buffer(out_buff_ptr, out_buff_len)
        };
        (6) => {
            |_, _, _, _, _, _, out_buff_ptr, out_buff_len| {
                write_id_to_buffer(out_buff_ptr, out_buff_len)
            }
        };
    }

    /// Generates a mock `returning` closure that returns INTERNAL_ERROR.
    /// Pass the total number of parameters of the host function.
    macro_rules! error_returning {
        (4) => {
            |_, _, _, _| INTERNAL_ERROR
        };
        (6) => {
            |_, _, _, _, _, _| INTERNAL_ERROR
        };
        (8) => {
            |_, _, _, _, _, _, _, _| INTERNAL_ERROR
        };
    }

    /// Generates a test module with success and error tests for a ledger entry ID function.
    ///
    /// Arguments:
    /// - `$mod_name`: name for the test module
    /// - `$expect_fn`: mock expectation method (e.g., `expect_accountroot_id`)
    /// - `$success_arity`: number of prefix params for write_ledger entry ID_returning (2, 4, or 6)
    /// - `$error_arity`: total number of params for error_returning (4, 6, or 8)
    /// - `$call_block`: block that sets up args and returns the ledger entry ID function call result
    macro_rules! id_test {
        ($mod_name:ident, $expect_fn:ident, $success_arity:tt, $error_arity:tt, $call_block:block) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_success() {
                    let mut mock = MockHostBindings::new();
                    mock.$expect_fn()
                        .times(1)
                        .returning(write_id_returning!($success_arity));
                    let _guard = setup_mock(mock);

                    let result = $call_block;
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap(), EXPECTED_ID);
                }

                #[test]
                fn test_error() {
                    let mut mock = MockHostBindings::new();
                    mock.$expect_fn()
                        .times(1)
                        .returning(error_returning!($error_arity));
                    let _guard = setup_mock(mock);

                    let result = $call_block;
                    assert!(result.is_err());
                    assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
                }
            }
        };
    }

    id_test!(accountroot_id_tests, expect_accountroot_id, 2, 4, {
        let account_id = AccountID::from([0xBB; 20]);
        accountroot_id(&account_id)
    });

    id_test!(check_id_tests, expect_check_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        check_id(&owner, 12345)
    });

    id_test!(delegate_id_tests, expect_delegate_id, 4, 6, {
        let account = AccountID::from([0xBB; 20]);
        let authorize = AccountID::from([0xBB; 20]);
        delegate_id(&account, &authorize)
    });

    id_test!(credential_id_tests, expect_credential_id, 6, 8, {
        let subject = AccountID::from([0xBB; 20]);
        let issuer = AccountID::from([0xBB; 20]);
        let cred_type: &[u8] = b"termsandconditions";
        credential_id(&subject, &issuer, cred_type)
    });

    id_test!(amm_id_tests, expect_amm_id, 4, 6, {
        use crate::types::issue::{Issue, XrpIssue};
        let issue1 = Issue::XRP(XrpIssue {});
        let issue2 = Issue::XRP(XrpIssue {});
        amm_id(&issue1, &issue2)
    });

    id_test!(deposit_preauth_id_tests, expect_deposit_preauth_id, 4, 6, {
        let account = AccountID::from([0xBB; 20]);
        let authorize = AccountID::from([0xBB; 20]);
        deposit_preauth_id(&account, &authorize)
    });

    id_test!(did_id_tests, expect_did_id, 2, 4, {
        let account_id = AccountID::from([0xBB; 20]);
        did_id(&account_id)
    });

    id_test!(escrow_id_tests, expect_escrow_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        escrow_id(&owner, 12345)
    });

    id_test!(trustline_id_tests, expect_trustline_id, 6, 8, {
        use crate::types::currency::Currency;
        let account1 = AccountID::from([0xBB; 20]);
        let account2 = AccountID::from([0xBB; 20]);
        let currency = Currency::from([0xBB; 20]);
        trustline_id(&account1, &account2, &currency)
    });

    id_test!(mpt_issuance_id_tests, expect_mpt_issuance_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        mpt_issuance_id(&owner, 12345)
    });

    id_test!(mptoken_id_tests, expect_mptoken_id, 4, 6, {
        use crate::types::mpt_id::MptId;
        let issuer = AccountID::from([0xBB; 20]);
        let mptid = MptId::new(1, issuer);
        let holder = AccountID::from([0xBB; 20]);
        mptoken_id(&mptid, &holder)
    });

    id_test!(nft_offer_id_tests, expect_nft_offer_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        nft_offer_id(&owner, 12345)
    });

    id_test!(offer_id_tests, expect_offer_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        offer_id(&owner, 12345)
    });

    id_test!(oracle_id_tests, expect_oracle_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        oracle_id(&owner, 12345)
    });

    id_test!(paychan_id_tests, expect_paychan_id, 6, 8, {
        let account = AccountID::from([0xBB; 20]);
        let destination = AccountID::from([0xBB; 20]);
        paychan_id(&account, &destination, 12345)
    });

    id_test!(
        permissioned_domain_id_tests,
        expect_permissioned_domain_id,
        4,
        6,
        {
            let account = AccountID::from([0xBB; 20]);
            permissioned_domain_id(&account, 12345)
        }
    );

    id_test!(signers_id_tests, expect_signers_id, 2, 4, {
        let account_id = AccountID::from([0xBB; 20]);
        signers_id(&account_id)
    });

    id_test!(ticket_id_tests, expect_ticket_id, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        ticket_id(&owner, 12345)
    });

    id_test!(vault_id_tests, expect_vault_id, 4, 6, {
        let account = AccountID::from([0xBB; 20]);
        vault_id(&account, 12345)
    });

    #[test]
    #[should_panic]
    fn test_wrong_size_panics() {
        let mut mock = MockHostBindings::new();

        // Return 16 instead of 32 — positive but wrong size
        mock.expect_accountroot_id()
            .times(1)
            .returning(|_, _, _, _| 16);

        let _guard = setup_mock(mock);

        let account_id = AccountID::from([0xBB; 20]);
        let _ = accountroot_id(&account_id);
    }
}
