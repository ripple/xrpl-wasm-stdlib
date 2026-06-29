#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::event::codec_v3::{EventBuffer, event_add};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;

#[unsafe(no_mangle)]
pub extern "C" fn events() -> i32 {
    let mut buf = EventBuffer::new();

    // STI_AMOUNT
    // const AMOUNT: [u8; 8] = [
    //     0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0
    // ];
    // let amount = TokenAmount::from_bytes(&AMOUNT).unwrap();
    // if event_add::<TokenAmount>(&mut buf, "amount", &amount).is_err() {
    //     return -1;
    // }

    // STI_CURRENCY
    // const CURRENCY: [u8; 20] = [
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x55, 0x53, 0x44, 0x00,
    //     0x00, 0x00, 0x00, 0x00
    // ];
    // if event_add::<Currency>(&mut buf, "currency", &Currency(CURRENCY)).is_err() {
    //     return -1;
    // }

    // STI_ACCOUNT
    const ACCOUNT: [u8; 20] = [
        0x59, 0x69, 0x15, 0xCF, 0xDE, 0xEE, 0x3A, 0x69, 0x5B, 0x3E, 0xFD, 0x6B, 0xDA, 0x9A, 0xC7,
        0x88, 0xA3, 0x68, 0xB7, 0xB,
    ];
    let account = AccountID(ACCOUNT);
    if event_add::<AccountID>(&mut buf, "destination", &account).is_err() {
        return -1;
    }

    // STI_UINT128
    // if event_add::<u128>(&mut buf, "uint128", &[0u8; 16]).is_err() {
    //     return -1;
    // }

    // STI_UINT16
    if event_add::<u16>(&mut buf, "uint16", &16).is_err() {
        return -1;
    }

    // STI_UINT160
    // if event_add::<u160>(&mut buf, "uint160", &[0u8; 20]).is_err() {
    //     return -1;
    // }

    // // STI_UINT192
    // if event_add::<u192>(&mut buf, "uint192", &[0u8; 24]).is_err() {
    //     return -1;
    // }

    // STI_UINT256
    // if event_add::<u256>(&mut buf, "uint256", &[0u8; 32]).is_err() {
    //     return -1;
    // }

    // STI_UINT32
    if event_add::<u32>(&mut buf, "uint32", &32).is_err() {
        return -1;
    }

    // STI_UINT64
    if event_add::<u64>(&mut buf, "uint64", &64).is_err() {
        return -1;
    }

    // STI_UINT8
    if event_add::<u8>(&mut buf, "uint8", &8).is_err() {
        return -1;
    }

    // STI_VL
    // if event_add(&mut buf, "vl", "Hello, World!").is_err() {
    //     return -1;
    // }

    // STI_ISSUE (XRP)
    // STI_ISSUE (IOU)
    // STI_ISSUE (MPT)

    if buf.emit("event1").is_err() {
        return -1;
    }
    0
}
