#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use crate::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_common_stdlib::host;
use xrpl_common_stdlib::host::trace::{DataRepr, trace, trace_acct, trace_data, trace_num};
use xrpl_common_stdlib::ledger_entry_ids;
use xrpl_common_stdlib::objects::ledger_object;
use xrpl_common_stdlib::sfield;
use xrpl_common_stdlib::sfield::SField;
use xrpl_common_stdlib::types::currency::Currency;
use xrpl_common_stdlib::types::issue::{IouIssue, Issue, XrpIssue};
use xrpl_common_stdlib::types::mpt_id::MptId;
use xrpl_escrow_stdlib::ledger_objects::current_escrow::CurrentEscrow;
use xrpl_escrow_stdlib::ledger_objects::current_escrow::get_current_escrow;
use xrpl_escrow_stdlib::ledger_objects::traits::CurrentEscrowFields;

pub fn object_exists<T, const CODE: i32>(
    id_result: Result<ledger_entry_ids::LedgerEntryIdBytes>,
    id_type: &str,
    field: SField<T, CODE>,
) -> Result<bool> {
    match id_result {
        Ok(id) => {
            let _ = trace_data(id_type, &id, DataRepr::AsHex);

            let slot = unsafe { host::cache_le(id.as_ptr(), id.len(), 0) };
            if slot < 0 {
                let _ = trace_num("Error: ", slot.into());
                return Err(Error::from_code(slot));
            }
            if CODE == 0 {
                let field_code: i32 = sfield::PreviousTxnID.into();
                let _ = trace_num("Getting field: ", field_code as i64);
                match ledger_object::get_field(slot, sfield::PreviousTxnID) {
                    Ok(data) => {
                        let _ = trace_data("Field data: ", &data.0, DataRepr::AsHex);
                    }
                    Err(result_code) => {
                        let _ = trace_num("Error getting field: ", result_code.into());
                        return Err(result_code);
                    }
                }
            } else {
                let field_code: i32 = field.into();
                let _ = trace_num("Getting field: ", field_code as i64);
                match ledger_object::get_field(slot, sfield::Account) {
                    Ok(data) => {
                        let _ = trace_data("Field data: ", &data.0, DataRepr::AsHex);
                    }
                    Err(result_code) => {
                        let _ = trace_num("Error getting field: ", result_code.into());
                        return Err(result_code);
                    }
                }
            }

            Ok(true)
        }
        Err(error) => {
            let _ = trace_num("Error getting ledger entry ID: ", error.into());
            Err(error)
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn escrow_finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");

    let escrow: CurrentEscrow = get_current_escrow();

    let account = escrow.get_account().unwrap_or_panic();
    let _ = trace_acct("Account:", &account);

    let destination = escrow.get_destination().unwrap_or_panic();
    let _ = trace_acct("Destination:", &destination);

    let mut seq = 5;

    macro_rules! check_object_exists {
        ($id:expr, $type:expr, $field:expr) => {
            match object_exists($id, $type, $field) {
                Ok(_exists) => {
                    // false isn't returned
                    let _ = trace(concat!(
                        $type,
                        " object exists, proceeding with escrow finish."
                    ));
                }
                Err(error) => {
                    let _ = trace_num("Current seq value:", seq.try_into().unwrap());
                    return error.code();
                }
            }
        };
    }

    let accountroot_id = ledger_entry_ids::accountroot_id(&account);
    check_object_exists!(accountroot_id, "Account", sfield::Account);

    let currency: &[u8; 3] = b"USD";
    let currency: Currency = Currency::from(*currency);
    let trustline_id = ledger_entry_ids::trustline_id(&account, &destination, &currency);
    check_object_exists!(trustline_id, "Trustline", sfield::Generic);
    seq += 1;

    let issue1 = Issue::XRP(XrpIssue {});
    let issue2 = Issue::IOU(IouIssue::new(destination, currency));
    check_object_exists!(
        ledger_entry_ids::amm_id(&issue1, &issue2),
        "AMM",
        sfield::Account
    );

    let check_id = ledger_entry_ids::check_id(&account, seq);
    check_object_exists!(check_id, "Check", sfield::Account);
    seq += 1;

    let cred_type: &[u8] = b"termsandconditions";
    let credential_id = ledger_entry_ids::credential_id(&account, &account, cred_type);
    check_object_exists!(credential_id, "Credential", sfield::Subject);
    seq += 1;

    let delegate_id = ledger_entry_ids::delegate_id(&account, &destination);
    check_object_exists!(delegate_id, "Delegate", sfield::Account);
    seq += 1;

    let deposit_preauth_id = ledger_entry_ids::deposit_preauth_id(&account, &destination);
    check_object_exists!(deposit_preauth_id, "DepositPreauth", sfield::Account);
    seq += 1;

    let did_id = ledger_entry_ids::did_id(&account);
    check_object_exists!(did_id, "DID", sfield::Account);
    seq += 1;

    let escrow_id = ledger_entry_ids::escrow_id(&account, seq);
    check_object_exists!(escrow_id, "Escrow", sfield::Account);
    seq += 1;

    let mpt_issuance_id = ledger_entry_ids::mpt_issuance_id(&account, seq);
    let mpt_id = MptId::new(seq, account);
    check_object_exists!(mpt_issuance_id, "MPTIssuance", sfield::Issuer);
    seq += 1;

    let mptoken_id = ledger_entry_ids::mptoken_id(&mpt_id, &destination);
    check_object_exists!(mptoken_id, "MPToken", sfield::Account);

    let nft_offer_id = ledger_entry_ids::nft_offer_id(&destination, 6);
    check_object_exists!(nft_offer_id, "NFTokenOffer", sfield::Owner);

    let offer_id = ledger_entry_ids::offer_id(&account, seq);
    check_object_exists!(offer_id, "Offer", sfield::Account);
    seq += 1;

    let oracle_id = ledger_entry_ids::oracle_id(&account, seq);
    check_object_exists!(oracle_id, "Oracle", sfield::Owner);
    seq += 1;

    let paychan_id = ledger_entry_ids::paychan_id(&account, &destination, seq);
    check_object_exists!(paychan_id, "PayChannel", sfield::Account);
    seq += 1;

    let pd_id = ledger_entry_ids::permissioned_domain_id(&account, seq);
    check_object_exists!(pd_id, "PermissionedDomain", sfield::Owner);
    seq += 1;

    let signers_id = ledger_entry_ids::signers_id(&account);
    check_object_exists!(signers_id, "SignerList", sfield::Generic);
    seq += 1;

    seq += 1; // ticket sequence number is one greater
    let ticket_id = ledger_entry_ids::ticket_id(&account, seq);
    check_object_exists!(ticket_id, "Ticket", sfield::Account);
    seq += 1;

    let vault_id = ledger_entry_ids::vault_id(&account, seq);
    check_object_exists!(vault_id, "Vault", sfield::Account);
    // seq += 1;

    1 // All ledger_entry_ids exist, finish the escrow.
}
