// #![cfg_attr(target_arch = "wasm32", no_std)]

// XRPL markers
const ARRAY_END: u8 = 0xF1;
const OBJECT_END: u8 = 0xE1;

// ============= Encoding helpers =============

#[inline]
fn encode_vl_length_inline(len: usize) -> (u8, u8, u8, u8) {
    if len <= 192 {
        (len as u8, 0, 0, 1)
    } else if len <= 12480 {
        let encoded = len - 193;
        (193 + (encoded >> 8) as u8, (encoded & 0xff) as u8, 0, 2)
    } else if len <= 918744 {
        let encoded = len - 12481;
        (
            241 + (encoded >> 16) as u8,
            ((encoded >> 8) & 0xff) as u8,
            (encoded & 0xff) as u8,
            3,
        )
    } else {
        (0, 0, 0, 0)
    }
}

// ============= SignerEntry =============
// Required: Account, SignerWeight
// Optional: WalletLocator

pub fn build_signer_entry(
    buffer: &mut [u8; 128],
    account: &[u8; 20],
    signer_weight: u16,
    wallet_locator: Option<&[u8; 32]>,
) -> usize {
    let mut pos = 0;

    // SignerEntry header: type=14, field=11
    buffer[pos] = (14 << 4) | 11;
    pos += 1;

    // SignerWeight (131075): type=2, field=3
    buffer[pos] = (2 << 4) | 3;
    pos += 1;
    buffer[pos..pos + 2].copy_from_slice(&signer_weight.to_be_bytes());
    pos += 2;

    // WalletLocator (524295) optional: type=8, field=7
    if let Some(locator) = wallet_locator {
        buffer[pos] = (8 << 4) | 7;
        pos += 1;
        buffer[pos..pos + 32].copy_from_slice(locator);
        pos += 32;
    }

    // Account (851969): type=13, field=1
    buffer[pos] = (13 << 4) | 1;
    pos += 1;
    buffer[pos] = 0x14;
    pos += 1;
    buffer[pos..pos + 20].copy_from_slice(account);
    pos += 20;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= Signer =============
// Required: Account, SigningPubKey, TxnSignature

pub fn build_signer(
    buffer: &mut [u8; 256],
    account: &[u8; 20],
    signing_pub_key: &[u8],
    txn_signature: &[u8],
) -> usize {
    let mut pos = 0;

    // Signer header: type=14, field=16
    buffer[pos] = 14 << 4; // type=14, field > 15
    pos += 1;
    buffer[pos] = 16;
    pos += 1;

    // SigningPubKey (786435): type=12, field=3
    buffer[pos] = (12 << 4) | 3;
    pos += 1;
    let (b0, b1, b2, len) = encode_vl_length_inline(signing_pub_key.len());
    buffer[pos] = b0;
    if len > 1 {
        buffer[pos + 1] = b1;
    }
    if len > 2 {
        buffer[pos + 2] = b2;
    }
    pos += len as usize;
    buffer[pos..pos + signing_pub_key.len()].copy_from_slice(signing_pub_key);
    pos += signing_pub_key.len();

    // TxnSignature (786436): type=12, field=4
    buffer[pos] = (12 << 4) | 4;
    pos += 1;
    let (b0, b1, b2, len) = encode_vl_length_inline(txn_signature.len());
    buffer[pos] = b0;
    if len > 1 {
        buffer[pos + 1] = b1;
    }
    if len > 2 {
        buffer[pos + 2] = b2;
    }
    pos += len as usize;
    buffer[pos..pos + txn_signature.len()].copy_from_slice(txn_signature);
    pos += txn_signature.len();

    // Account (851969): type=13, field=1
    buffer[pos] = (13 << 4) | 1;
    pos += 1;
    buffer[pos] = 0x14;
    pos += 1;
    buffer[pos..pos + 20].copy_from_slice(account);
    pos += 20;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= Majority =============
// Required: Amendment, CloseTime

pub fn build_majority(buffer: &mut [u8; 64], amendment: &[u8; 32], close_time: u32) -> usize {
    let mut pos = 0;

    // Majority header: type=14, field=18
    buffer[pos] = 14 << 4;
    pos += 1;
    buffer[pos] = 18;
    pos += 1;

    // CloseTime (196615): type=3, field=7
    buffer[pos] = (3 << 4) | 7;
    pos += 1;
    buffer[pos..pos + 4].copy_from_slice(&close_time.to_be_bytes());
    pos += 4;

    // Amendment (524307): type=8, field=19
    buffer[pos] = 8 << 4;
    pos += 1;
    buffer[pos] = 19;
    pos += 1;
    buffer[pos..pos + 32].copy_from_slice(amendment);
    pos += 32;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= DisabledValidator =============
// Required: PublicKey, FirstLedgerSequence

pub fn build_disabled_validator(
    buffer: &mut [u8; 128],
    public_key: &[u8],
    first_ledger_sequence: u32,
) -> usize {
    let mut pos = 0;

    // DisabledValidator header: type=14, field=19
    buffer[pos] = 14 << 4;
    pos += 1;
    buffer[pos] = 19;
    pos += 1;

    // FirstLedgerSequence (196634): type=3, field=26
    buffer[pos] = 3 << 4;
    pos += 1;
    buffer[pos] = 26;
    pos += 1;
    buffer[pos..pos + 4].copy_from_slice(&first_ledger_sequence.to_be_bytes());
    pos += 4;

    // PublicKey (786433): type=12, field=1
    buffer[pos] = (12 << 4) | 1;
    pos += 1;
    let (b0, b1, b2, len) = encode_vl_length_inline(public_key.len());
    buffer[pos] = b0;
    if len > 1 {
        buffer[pos + 1] = b1;
    }
    if len > 2 {
        buffer[pos + 2] = b2;
    }
    pos += len as usize;
    buffer[pos..pos + public_key.len()].copy_from_slice(public_key);
    pos += public_key.len();

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= NFToken =============
// Required: NFTokenID
// Optional: URI

pub fn build_nftoken(buffer: &mut [u8; 256], nftoken_id: &[u8; 32], uri: Option<&[u8]>) -> usize {
    let mut pos = 0;

    // NFToken header: type=14, field=12
    buffer[pos] = (14 << 4) | 12;
    pos += 1;

    // NFTokenID (524298): type=8, field=10
    buffer[pos] = (8 << 4) | 10;
    pos += 1;
    buffer[pos..pos + 32].copy_from_slice(nftoken_id);
    pos += 32;

    // URI (786437) optional: type=12, field=5
    if let Some(u) = uri {
        buffer[pos] = (12 << 4) | 5;
        pos += 1;
        let (b0, b1, b2, len) = encode_vl_length_inline(u.len());
        buffer[pos] = b0;
        if len > 1 {
            buffer[pos + 1] = b1;
        }
        if len > 2 {
            buffer[pos + 2] = b2;
        }
        pos += len as usize;
        buffer[pos..pos + u.len()].copy_from_slice(u);
        pos += u.len();
    }

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= VoteEntry =============
// Required: Account, VoteWeight
// Default: TradingFee

pub fn build_vote_entry(
    buffer: &mut [u8; 64],
    account: &[u8; 20],
    vote_weight: u32,
    trading_fee: Option<u16>,
) -> usize {
    let mut pos = 0;

    // VoteEntry header: type=14, field=25
    buffer[pos] = 14 << 4;
    pos += 1;
    buffer[pos] = 25;
    pos += 1;

    // TradingFee (131077) if provided: type=2, field=5
    if let Some(fee) = trading_fee {
        buffer[pos] = (2 << 4) | 5;
        pos += 1;
        buffer[pos..pos + 2].copy_from_slice(&fee.to_be_bytes());
        pos += 2;
    }

    // VoteWeight (196656): type=3, field=48
    buffer[pos] = 3 << 4;
    pos += 1;
    buffer[pos] = 48;
    pos += 1;
    buffer[pos..pos + 4].copy_from_slice(&vote_weight.to_be_bytes());
    pos += 4;

    // Account (851969): type=13, field=1
    buffer[pos] = (13 << 4) | 1;
    pos += 1;
    buffer[pos] = 0x14;
    pos += 1;
    buffer[pos..pos + 20].copy_from_slice(account);
    pos += 20;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= AuthAccount =============
// Required: Account

pub fn build_auth_account(buffer: &mut [u8; 32], account: &[u8; 20]) -> usize {
    let mut pos = 0;

    // AuthAccount header: type=14, field=27
    buffer[pos] = 14 << 4;
    pos += 1;
    buffer[pos] = 27;
    pos += 1;

    // Account (851969): type=13, field=1
    buffer[pos] = (13 << 4) | 1;
    pos += 1;
    buffer[pos] = 0x14;
    pos += 1;
    buffer[pos..pos + 20].copy_from_slice(account);
    pos += 20;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= Credential =============
// Required: Issuer, CredentialType

pub fn build_credential(
    buffer: &mut [u8; 256],
    issuer: &[u8; 20],
    credential_type: &[u8],
) -> usize {
    let mut pos = 0;

    // Credential header: type=14, field=33
    buffer[pos] = 14 << 4;
    pos += 1;
    buffer[pos] = 33;
    pos += 1;

    // CredentialType (786463): type=12, field=31
    buffer[pos] = 12 << 4;
    pos += 1;
    buffer[pos] = 31;
    pos += 1;
    let (b0, b1, b2, len) = encode_vl_length_inline(credential_type.len());
    buffer[pos] = b0;
    if len > 1 {
        buffer[pos + 1] = b1;
    }
    if len > 2 {
        buffer[pos + 2] = b2;
    }
    pos += len as usize;
    buffer[pos..pos + credential_type.len()].copy_from_slice(credential_type);
    pos += credential_type.len();

    // Issuer (851972): type=13, field=4
    buffer[pos] = (13 << 4) | 4;
    pos += 1;
    buffer[pos] = 0x14;
    pos += 1;
    buffer[pos..pos + 20].copy_from_slice(issuer);
    pos += 20;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= Permission =============
// Required: PermissionValue

pub fn build_permission(buffer: &mut [u8; 16], permission_value: u32) -> usize {
    let mut pos = 0;

    // Permission header: type=14, field=15
    buffer[pos] = (14 << 4) | 15;
    pos += 1;

    // PermissionValue (196660): type=3, field=52
    buffer[pos] = 3 << 4;
    pos += 1;
    buffer[pos] = 52;
    pos += 1;
    buffer[pos..pos + 4].copy_from_slice(&permission_value.to_be_bytes());
    pos += 4;

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= Memo (most common) =============

pub fn build_memo(
    buffer: &mut [u8; 256],
    memo_type: Option<&[u8]>,
    memo_data: Option<&[u8]>,
    memo_format: Option<&[u8]>,
) -> usize {
    let mut pos = 0;

    // Memo header: type=14, field=10
    buffer[pos] = (14 << 4) | 10;
    pos += 1;

    // MemoType (786444): type=12, field=12
    if let Some(mt) = memo_type {
        buffer[pos] = (7 << 4) | 12;
        pos += 1;
        let (b0, b1, b2, len) = encode_vl_length_inline(mt.len());
        buffer[pos] = b0;
        if len > 1 {
            buffer[pos + 1] = b1;
        }
        if len > 2 {
            buffer[pos + 2] = b2;
        }
        pos += len as usize;
        buffer[pos..pos + mt.len()].copy_from_slice(mt);
        pos += mt.len();
    }

    // MemoData (786445): type=12, field=13
    if let Some(md) = memo_data {
        buffer[pos] = (7 << 4) | 13;
        pos += 1;
        let (b0, b1, b2, len) = encode_vl_length_inline(md.len());
        buffer[pos] = b0;
        if len > 1 {
            buffer[pos + 1] = b1;
        }
        if len > 2 {
            buffer[pos + 2] = b2;
        }
        pos += len as usize;
        buffer[pos..pos + md.len()].copy_from_slice(md);
        pos += md.len();
    }

    // MemoFormat (786446): type=12, field=14
    if let Some(mf) = memo_format {
        buffer[pos] = (7 << 4) | 14;
        pos += 1;
        let (b0, b1, b2, len) = encode_vl_length_inline(mf.len());
        buffer[pos] = b0;
        if len > 1 {
            buffer[pos + 1] = b1;
        }
        if len > 2 {
            buffer[pos + 2] = b2;
        }
        pos += len as usize;
        buffer[pos..pos + mf.len()].copy_from_slice(mf);
        pos += mf.len();
    }

    buffer[pos] = OBJECT_END;
    pos + 1
}

// ============= Array builder helper =============

pub fn build_array<const N: usize>(buffer: &mut [u8; N], objects: &[&[u8]]) -> usize {
    let mut pos = 0;

    for obj in objects {
        buffer[pos..pos + obj.len()].copy_from_slice(obj);
        pos += obj.len();
    }

    buffer[pos] = ARRAY_END;
    pos + 1
}
