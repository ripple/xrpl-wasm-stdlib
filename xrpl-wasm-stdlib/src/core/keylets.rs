use crate::core::types::account_id::AccountID;
use crate::core::types::currency::Currency;
use crate::core::types::issue::Issue;
use crate::core::types::mpt_id::MptId;
use crate::host;
use crate::host::Result;
use crate::host::error_codes::match_result_code_with_expected_bytes;

pub const XRPL_KEYLET_SIZE: usize = 32;
// Type aliases for specific keylets, all currently using the same underlying array type.
pub type KeyletBytes = [u8; XRPL_KEYLET_SIZE];

/// Generates an account keylet for a given XRP Ledger account.
///
/// Account keylets are used to reference account entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte account keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::account_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::account_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match account_keylet(&account){
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn account_keylet(account_id: &AccountID) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::account_keylet(
            account_id.0.as_ptr(), // Assuming AccountID is a tuple struct like AccountID(bytes)
            account_id.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an AMM keylet for a given pair of accounts and currency code.
///
/// An AMM keylet is used to reference AMM entries in the XRP Ledger.
///
/// # Arguments
///
/// * `issue1` - The first Issue in the AMM relationship
/// * `issue2` - The second Issue in the AMM relationship
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte AMM keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::amm_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::issue::{Issue, XrpIssue, IouIssue};
/// use xrpl_wasm_stdlib::core::types::currency::Currency;
/// use xrpl_wasm_stdlib::core::keylets::amm_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let issue1: Issue = Issue::XRP(XrpIssue {});
///  let issuer: AccountID =
///    AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///  let currency = b"RLUSD\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"; // RLUSD currency code
///  let currency: Currency = Currency::from(*currency);
///  let issue2 = Issue::IOU(IouIssue::new(issuer, currency));
///  match amm_keylet(&issue1, &issue2) {
///    xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///      let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///    }
///    xrpl_wasm_stdlib::host::Result::Err(e) => {
///      let _ = trace_num("Error assembling keylet", e.code() as i64);
///    }
///  }
///  Ok(())
/// }
/// ```
pub fn amm_keylet(issue1: &Issue, issue2: &Issue) -> Result<KeyletBytes> {
    let issue1_bytes = issue1.as_bytes();
    let issue2_bytes = issue2.as_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::amm_keylet(
            issue1_bytes.as_ptr(),
            issue1_bytes.len(),
            issue2_bytes.as_ptr(),
            issue2_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an check keylet for a given owner and sequence in the XRP Ledger.
///
/// Check keylets are used to reference check entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the check owner's account
/// * `seq` - The account sequence associated with the check entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte check keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::check_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::check_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match check_keylet(&owner, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn check_keylet(owner: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::check_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a credential keylet for a given subject, issuer, and credential type.
///
/// A credential keylet is used to reference credential entries in the XRP Ledger.
///
/// # Arguments
///
/// * `subject` - The AccountID of the subject for whom the credential is issued
/// * `issuer` - The AccountID of the entity issuing the credential
/// * `credential_type` - A byte slice representing the type of credential
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte credential keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::credential_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::credential_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let subject: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let issuer: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let cred_type: &[u8] = b"termsandconditions";
///     match credential_keylet(&subject, &issuer, cred_type) {
///       xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///         let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///       }
///       xrpl_wasm_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling keylet", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn credential_keylet(
    subject: &AccountID,
    issuer: &AccountID,
    credential_type: &[u8],
) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::credential_keylet(
            subject.0.as_ptr(),
            subject.0.len(),
            issuer.0.as_ptr(),
            issuer.0.len(),
            credential_type.as_ptr(),
            credential_type.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a delegate keylet for a given given account and authorized account.
///
/// A delegate keylet is used to reference delegate entries in the XRP Ledger.
///
/// # Arguments
///
/// * `account` - The AccountID of the account that is delegating permissions
/// * `authorize` - The AccountID of the account that is delegated to
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte delegate keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::delegate_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::delegate_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let account: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let authorize: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     match delegate_keylet(&account, &authorize) {
///       xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///         let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///       }
///       xrpl_wasm_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling keylet", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn delegate_keylet(account: &AccountID, authorize: &AccountID) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::delegate_keylet(
            account.0.as_ptr(),
            account.0.len(),
            authorize.0.as_ptr(),
            authorize.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a deposit preauth keylet for a given account and authorized account.
///
/// A deposit preauth keylet is used to reference deposit preauth entries in the XRP Ledger.
///
/// # Arguments
///
/// * `account` - The AccountID of the account that is doing the pre-authorizing
/// * `authorize` - The AccountID of the account that is pre-authorizing
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte deposit preauth keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::deposit_preauth_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::deposit_preauth_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let account: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let authorize: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     match deposit_preauth_keylet(&account, &authorize) {
///       xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///         let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///       }
///       xrpl_wasm_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling keylet", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn deposit_preauth_keylet(account: &AccountID, authorize: &AccountID) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::deposit_preauth_keylet(
            account.0.as_ptr(),
            account.0.len(),
            authorize.0.as_ptr(),
            authorize.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a DID keylet for a given XRP Ledger account.
///
/// DID keylets are used to reference DID entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte DID keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::did_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::did_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match did_keylet(&account){
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn did_keylet(account_id: &AccountID) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::did_keylet(
            account_id.0.as_ptr(),
            account_id.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an escrow keylet for a given owner and sequence in the XRP Ledger.
///
/// Escrow keylets are used to reference escrow entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the escrow owner's account
/// * `seq` - The account sequence associated with the escrow entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte escrow keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::escrow_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::escrow_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match escrow_keylet(&owner, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn escrow_keylet(owner: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::escrow_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a trustline keylet for a given pair of accounts and currency code.
///
/// A trustline keylet is used to reference trustline entries in the XRP Ledger.
///
/// # Arguments
///
/// * `account` - The first AccountID in the trustline relationship
/// * `account2` - The second AccountID in the trustline relationship
/// * `currency` - The Currency for the trustline
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte trustline keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::line_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::currency::Currency;
/// use xrpl_wasm_stdlib::core::keylets::line_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let account1: AccountID =
///    AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///  let account2: AccountID =
///    AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///  let currency = b"RLUSD\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"; // RLUSD currency code
///  let currency: Currency = Currency::from(*currency);
///  match line_keylet(&account1, &account2, &currency) {
///    xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///      let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///    }
///    xrpl_wasm_stdlib::host::Result::Err(e) => {
///      let _ = trace_num("Error assembling keylet", e.code() as i64);
///    }
///  }
///  Ok(())
/// }
/// ```
pub fn line_keylet(
    account1: &AccountID,
    account2: &AccountID,
    currency: &Currency,
) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::line_keylet(
            account1.0.as_ptr(),
            account1.0.len(),
            account2.0.as_ptr(),
            account2.0.len(),
            currency.0.as_ptr(),
            currency.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an MPT issuance keylet for a given owner and sequence in the XRP Ledger.
///
/// MPT issuance keylets are used to reference MPT issuance entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the MPT issuer's account
/// * `seq` - The account sequence associated with the MPT issuance entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte MPT issuance keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::mpt_issuance_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::mpt_issuance_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match mpt_issuance_keylet(&owner, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn mpt_issuance_keylet(owner: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::mpt_issuance_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an MPToken keylet for a given MPT ID and holder.
///
/// An MPToken keylet is used to reference MPToken entries in the XRP Ledger.
///
/// # Arguments
///
/// * `mptid` - The MPT ID that the MPToken is associated with
/// * `holder` - The AccountID of the account that holds the MPToken
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte MPToken keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::mptoken_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::types::mpt_id::MptId;
/// use xrpl_wasm_stdlib::core::keylets::mptoken_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let issuer: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let mptid: MptId = MptId::new(1, issuer);
///     let holder: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     match mptoken_keylet(&mptid, &holder) {
///       xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///         let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///       }
///       xrpl_wasm_stdlib::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling keylet", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn mptoken_keylet(mptid: &MptId, holder: &AccountID) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::mptoken_keylet(
            mptid.as_bytes().as_ptr(),
            mptid.as_bytes().len(),
            holder.0.as_ptr(),
            holder.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an NFT offer keylet for a given owner and sequence in the XRP Ledger.
///
/// NFT offer keylets are used to reference NFT offer entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the NFT offer owner's account
/// * `seq` - The account sequence associated with the NFT offer entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte NFT offer keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::nft_offer_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::nft_offer_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match nft_offer_keylet(&owner, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn nft_offer_keylet(owner: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::nft_offer_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an offer keylet for a given owner and sequence in the XRP Ledger.
///
/// Offer keylets are used to reference offer entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the offer owner's account
/// * `seq` - The account sequence associated with the offer entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte offer keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::offer_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::offer_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match offer_keylet(&owner, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn offer_keylet(owner: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::offer_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an oracle keylet for a given owner and document ID in the XRP Ledger.
///
/// Oracle keylets are used to reference oracle entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the oracle owner's account
/// * `document_id` - An integer identifier for the oracle document
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte oracle keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::oracle_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::oracle_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let document_id = 12345;
///   match oracle_keylet(&owner, document_id) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn oracle_keylet(owner: &AccountID, document_id: u32) -> Result<KeyletBytes> {
    let document_id_bytes = document_id.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::oracle_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            document_id_bytes.as_ptr(),
            document_id_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a payment channel keylet for a given owner and sequence in the XRP Ledger.
///
/// Payment channel keylets are used to reference payment channel entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account` - Reference to an `AccountID` representing the payment channel sender's account
/// * `destination` - Reference to an `AccountID` representing the payment channel's destination
/// * `seq` - The account sequence associated with the payment channel entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte payment channel keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::paychan_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::paychan_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let destination: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match paychan_keylet(&account, &destination, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn paychan_keylet(
    account: &AccountID,
    destination: &AccountID,
    seq: u32,
) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::paychan_keylet(
            account.0.as_ptr(),
            account.0.len(),
            destination.0.as_ptr(),
            destination.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a permissioned domain keylet for a given owner and sequence in the XRP Ledger.
///
/// Permissioned domain keylets are used to reference permissioned domain entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account` - Reference to an `AccountID` representing the permissioned domain's owner
/// * `seq` - The account sequence associated with the permissioned domain entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte permissioned domain keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::permissioned_domain_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::permissioned_domain_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match permissioned_domain_keylet(&account, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn permissioned_domain_keylet(account: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::permissioned_domain_keylet(
            account.0.as_ptr(),
            account.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a signer entry keylet for a given XRP Ledger account.
///
/// signer entry keylets are used to reference signer entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte signer entry keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::signers_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::signers_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match signers_keylet(&account){
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn signers_keylet(account_id: &AccountID) -> Result<KeyletBytes> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::signers_keylet(
            account_id.0.as_ptr(),
            account_id.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a ticket keylet for a given owner and sequence in the XRP Ledger.
///
/// Ticket keylets are used to reference ticket entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the ticket owner's account
/// * `seq` - The account sequence associated with the ticket entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte ticket keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::ticket_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::ticket_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match ticket_keylet(&owner, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn ticket_keylet(owner: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::ticket_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a vault keylet for a given owner and sequence in the XRP Ledger.
///
/// Vault keylets are used to reference vault entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account` - Reference to an `AccountID` representing the vault's owner
/// * `seq` - The account sequence associated with the vault entry
///
/// # Returns
///
/// * `Result<KeyletBytes>` - On success, returns a 32-byte vault keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::vault_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_wasm_stdlib::core::types::account_id::AccountID;
/// use xrpl_wasm_stdlib::core::keylets::vault_keylet;
/// use xrpl_wasm_stdlib::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let sequence = 12345;
///   match vault_keylet(&account, sequence) {
///     xrpl_wasm_stdlib::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_wasm_stdlib::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn vault_keylet(account: &AccountID, seq: u32) -> Result<KeyletBytes> {
    let seq_bytes = seq.to_le_bytes();
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::vault_keylet(
            account.0.as_ptr(),
            account.0.len(),
            seq_bytes.as_ptr(),
            seq_bytes.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generic helper function to create a keylet by calling a host function.
///
/// This function handles the common tasks of:
/// - Initializing the keylet output buffer.
/// - Invoking the provided `host_call` closure (which performs the unsafe host FFI call).
/// - Converting the host call's `i32` result code into a `Result<KeyletBytes, Error>`.
///
/// # Arguments
///
/// * `host_call`: A closure that takes a mutable pointer to the output buffer (`*mut u8`)
///   and its length (`usize`), performs the specific host FFI call, and returns an `i32` status
///   code.
fn create_keylet_from_host_call<F>(host_call: F) -> Result<KeyletBytes>
where
    F: FnOnce(*mut u8, usize) -> i32,
{
    let mut keylet_buffer: KeyletBytes = [0; XRPL_KEYLET_SIZE];
    let result_code: i32 = host_call(keylet_buffer.as_mut_ptr(), keylet_buffer.len());

    match_result_code_with_expected_bytes(result_code, XRPL_KEYLET_SIZE, || keylet_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::error_codes::INTERNAL_ERROR;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;

    const EXPECTED_KEYLET: KeyletBytes = [0xCC; XRPL_KEYLET_SIZE];

    /// Writes `0xCC` into the output buffer and returns `XRPL_KEYLET_SIZE` as success.
    fn write_keylet_to_buffer(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
        assert_eq!(out_buff_len, XRPL_KEYLET_SIZE);
        unsafe {
            for i in 0..XRPL_KEYLET_SIZE {
                *out_buff_ptr.add(i) = 0xCC;
            }
        }
        XRPL_KEYLET_SIZE as i32
    }

    /// Generates a mock `returning` closure that delegates to `write_keylet_to_buffer`.
    /// Pass the number of prefix parameters (before the out_buff_ptr/out_buff_len pair)
    /// to match the host function arity.
    macro_rules! write_keylet_returning {
        (2) => {
            |_, _, out_buff_ptr, out_buff_len| write_keylet_to_buffer(out_buff_ptr, out_buff_len)
        };
        (4) => {
            |_, _, _, _, out_buff_ptr, out_buff_len| {
                write_keylet_to_buffer(out_buff_ptr, out_buff_len)
            }
        };
        (6) => {
            |_, _, _, _, _, _, out_buff_ptr, out_buff_len| {
                write_keylet_to_buffer(out_buff_ptr, out_buff_len)
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

    /// Generates a test module with success and error tests for a keylet function.
    ///
    /// Arguments:
    /// - `$mod_name`: name for the test module
    /// - `$expect_fn`: mock expectation method (e.g., `expect_account_keylet`)
    /// - `$success_arity`: number of prefix params for write_keylet_returning (2, 4, or 6)
    /// - `$error_arity`: total number of params for error_returning (4, 6, or 8)
    /// - `$call_block`: block that sets up args and returns the keylet function call result
    macro_rules! keylet_test {
        ($mod_name:ident, $expect_fn:ident, $success_arity:tt, $error_arity:tt, $call_block:block) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_success() {
                    let mut mock = MockHostBindings::new();
                    mock.$expect_fn()
                        .times(1)
                        .returning(write_keylet_returning!($success_arity));
                    let _guard = setup_mock(mock);

                    let result = $call_block;
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap(), EXPECTED_KEYLET);
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

    keylet_test!(account_keylet_tests, expect_account_keylet, 2, 4, {
        let account_id = AccountID::from([0xBB; 20]);
        account_keylet(&account_id)
    });

    keylet_test!(check_keylet_tests, expect_check_keylet, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        check_keylet(&owner, 12345)
    });

    keylet_test!(delegate_keylet_tests, expect_delegate_keylet, 4, 6, {
        let account = AccountID::from([0xBB; 20]);
        let authorize = AccountID::from([0xBB; 20]);
        delegate_keylet(&account, &authorize)
    });

    keylet_test!(credential_keylet_tests, expect_credential_keylet, 6, 8, {
        let subject = AccountID::from([0xBB; 20]);
        let issuer = AccountID::from([0xBB; 20]);
        let cred_type: &[u8] = b"termsandconditions";
        credential_keylet(&subject, &issuer, cred_type)
    });

    keylet_test!(amm_keylet_tests, expect_amm_keylet, 4, 6, {
        use crate::core::types::issue::{Issue, XrpIssue};
        let issue1 = Issue::XRP(XrpIssue {});
        let issue2 = Issue::XRP(XrpIssue {});
        amm_keylet(&issue1, &issue2)
    });

    keylet_test!(
        deposit_preauth_keylet_tests,
        expect_deposit_preauth_keylet,
        4,
        6,
        {
            let account = AccountID::from([0xBB; 20]);
            let authorize = AccountID::from([0xBB; 20]);
            deposit_preauth_keylet(&account, &authorize)
        }
    );

    keylet_test!(did_keylet_tests, expect_did_keylet, 2, 4, {
        let account_id = AccountID::from([0xBB; 20]);
        did_keylet(&account_id)
    });

    keylet_test!(escrow_keylet_tests, expect_escrow_keylet, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        escrow_keylet(&owner, 12345)
    });

    keylet_test!(line_keylet_tests, expect_line_keylet, 6, 8, {
        use crate::core::types::currency::Currency;
        let account1 = AccountID::from([0xBB; 20]);
        let account2 = AccountID::from([0xBB; 20]);
        let currency = Currency::from([0xBB; 20]);
        line_keylet(&account1, &account2, &currency)
    });

    keylet_test!(
        mpt_issuance_keylet_tests,
        expect_mpt_issuance_keylet,
        4,
        6,
        {
            let owner = AccountID::from([0xBB; 20]);
            mpt_issuance_keylet(&owner, 12345)
        }
    );

    keylet_test!(mptoken_keylet_tests, expect_mptoken_keylet, 4, 6, {
        use crate::core::types::mpt_id::MptId;
        let issuer = AccountID::from([0xBB; 20]);
        let mptid = MptId::new(1, issuer);
        let holder = AccountID::from([0xBB; 20]);
        mptoken_keylet(&mptid, &holder)
    });

    keylet_test!(nft_offer_keylet_tests, expect_nft_offer_keylet, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        nft_offer_keylet(&owner, 12345)
    });

    keylet_test!(offer_keylet_tests, expect_offer_keylet, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        offer_keylet(&owner, 12345)
    });

    keylet_test!(oracle_keylet_tests, expect_oracle_keylet, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        oracle_keylet(&owner, 12345)
    });

    keylet_test!(paychan_keylet_tests, expect_paychan_keylet, 6, 8, {
        let account = AccountID::from([0xBB; 20]);
        let destination = AccountID::from([0xBB; 20]);
        paychan_keylet(&account, &destination, 12345)
    });

    keylet_test!(
        permissioned_domain_keylet_tests,
        expect_permissioned_domain_keylet,
        4,
        6,
        {
            let account = AccountID::from([0xBB; 20]);
            permissioned_domain_keylet(&account, 12345)
        }
    );

    keylet_test!(signers_keylet_tests, expect_signers_keylet, 2, 4, {
        let account_id = AccountID::from([0xBB; 20]);
        signers_keylet(&account_id)
    });

    keylet_test!(ticket_keylet_tests, expect_ticket_keylet, 4, 6, {
        let owner = AccountID::from([0xBB; 20]);
        ticket_keylet(&owner, 12345)
    });

    keylet_test!(vault_keylet_tests, expect_vault_keylet, 4, 6, {
        let account = AccountID::from([0xBB; 20]);
        vault_keylet(&account, 12345)
    });

    #[test]
    fn test_wrong_size_returns_internal_error() {
        let mut mock = MockHostBindings::new();

        // Return 16 instead of 32 — positive but wrong size
        mock.expect_account_keylet()
            .times(1)
            .returning(|_, _, _, _| 16);

        let _guard = setup_mock(mock);

        let account_id = AccountID::from([0xBB; 20]);
        let result = account_keylet(&account_id);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().code(), INTERNAL_ERROR);
    }
}
