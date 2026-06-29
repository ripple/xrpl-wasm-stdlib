use xrpl_escrow_stdlib::core::types::account_id::AccountID;
use xrpl_escrow_stdlib::r_address;

#[test]
fn r_address_expands_to_const_account_id() {
    const ACCOUNT: AccountID = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
    assert_eq!(
        ACCOUNT.0,
        [
            0xb5, 0xf7, 0x62, 0x79, 0x8a, 0x53, 0xd5, 0x43, 0xa0, 0x14, 0xca, 0xf8, 0xb2, 0x97,
            0xcf, 0xf8, 0xf2, 0xf9, 0x37, 0xe8,
        ]
    );
}
