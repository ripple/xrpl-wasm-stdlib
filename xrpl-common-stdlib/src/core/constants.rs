use crate::core::types::account_id::AccountID;

/// The 20 bytes of Account Zero (rrrrrrrrrrrrrrrrrrrrrhoLvTp)
pub const ACCOUNT_ZERO: AccountID = AccountID([0u8; 20]);

/// The 20 bytes of Account One (rrrrrrrrrrrrrrrrrrrrBZbvji)
pub const ACCOUNT_ONE: AccountID = {
    // Create a mutable array *only* during compile-time evaluation
    let mut arr = [0x00; 20];
    arr[19] = 0x01;
    // The final value of the block is the initialized array
    AccountID(arr)
};

/// Indivisible unit of XRP
pub const ONE_DROP: u64 = 1;

/// 100 billion XRP
pub const MAX_XRP: u64 = 100_000_000_000u64;
/// Maximum possible drops of XRP
pub const MAX_DROPS: u64 = MAX_XRP * 1_000_000;
