#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_escrow_stdlib::core::current_tx::escrow_finish::get_current_escrow_finish;
use xrpl_escrow_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_escrow_stdlib::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_escrow_stdlib::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_escrow_stdlib::core::locator::Locator;
use xrpl_escrow_stdlib::core::types::account_id::AccountID;
use xrpl_escrow_stdlib::core::types::contract_data::ContractData;
use xrpl_escrow_stdlib::host::get_parent_ledger_time;
use xrpl_escrow_stdlib::host::get_tx_nested_field;
use xrpl_escrow_stdlib::host::trace::trace_num;
use xrpl_escrow_stdlib::host::{Error, Result, Result::Err, Result::Ok};
use xrpl_escrow_stdlib::sfield;

macro_rules! try_or_trace {
    ($e:expr, $label:literal) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                let _ = trace_num($label, e.code() as i64);
                return e.code();
            }
        }
    };
}

// ── Intent ────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq)]
enum Intent {
    Confirm,
    Deconfirm,
    Dispute,
    Undispute,
}

impl Intent {
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::Confirm),
            1 => Some(Self::Deconfirm),
            2 => Some(Self::Dispute),
            3 => Some(Self::Undispute),
            _ => None,
        }
    }
}

// ── Arbitrator rulings ────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq)]
enum ArbRuling {
    ForFreelancer, // INTENT_CONFIRM — release funds to freelancer
    ForClient,     // INTENT_DISPUTE — lock escrow until CancelAfter
}

impl ArbRuling {
    fn from_intent(intent: Intent) -> Option<Self> {
        match intent {
            Intent::Confirm => Some(Self::ForFreelancer),
            Intent::Dispute => Some(Self::ForClient),
            _ => None,
        }
    }
}

// ── Roles & dispute state ─────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq)]
enum Party {
    Client,
    Freelancer,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Role {
    Client,
    Freelancer,
    Arbitrator,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum DisputeState {
    None,
    ActiveBy(Party),
    ArbLocked,
}

// ── Escrow state wrapper ───────────────────────────────────────────────────────
//
// 27-byte Data layout:
//   0..20  arbitrator AccountID
//  20..24  deadline u32 LE (Ripple epoch seconds)
//     24   client_confirmed  (0/1)
//     25   freelancer_confirmed (0/1)
//     26   disputing_party (0=none, 1=client, 2=freelancer, 3=arb_locked)

struct State {
    inner: ContractData,
}

impl State {
    const SIZE: usize = 27;
    const ARBITRATOR: core::ops::Range<usize> = 0..20;
    const DEADLINE: core::ops::Range<usize> = 20..24;
    const CLIENT_CONFIRMED: usize = 24;
    const FREELANCER_CONFIRMED: usize = 25;
    const DISPUTING_PARTY: usize = 26;

    fn load(escrow: &CurrentEscrow) -> Result<Self> {
        let inner = match escrow.get_data() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        if inner.len < Self::SIZE {
            return Err(Error::InvalidParams);
        }
        Ok(Self { inner })
    }

    fn arbitrator(&self) -> AccountID {
        let mut buf = [0u8; 20];
        buf.copy_from_slice(&self.inner.data[Self::ARBITRATOR]);
        AccountID(buf)
    }

    fn deadline(&self) -> u32 {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.inner.data[Self::DEADLINE]);
        u32::from_le_bytes(bytes)
    }

    fn client_confirmed(&self) -> bool {
        self.inner.data[Self::CLIENT_CONFIRMED] != 0
    }

    fn freelancer_confirmed(&self) -> bool {
        self.inner.data[Self::FREELANCER_CONFIRMED] != 0
    }

    fn set_confirmation(&mut self, party: Party, confirmed: bool) {
        let idx = match party {
            Party::Client => Self::CLIENT_CONFIRMED,
            Party::Freelancer => Self::FREELANCER_CONFIRMED,
        };
        self.inner.data[idx] = confirmed as u8;
    }

    fn dispute(&self) -> DisputeState {
        if self.inner.data[Self::DISPUTING_PARTY] == 0 {
            return DisputeState::None;
        }
        match self.inner.data[Self::DISPUTING_PARTY] {
            1 => DisputeState::ActiveBy(Party::Client),
            2 => DisputeState::ActiveBy(Party::Freelancer),
            3 => DisputeState::ArbLocked,
            _ => DisputeState::None,
        }
    }

    fn set_dispute(&mut self, ds: DisputeState) {
        self.inner.data[Self::DISPUTING_PARTY] = match ds {
            DisputeState::None => 0,
            DisputeState::ActiveBy(Party::Client) => 1,
            DisputeState::ActiveBy(Party::Freelancer) => 2,
            DisputeState::ArbLocked => 3,
        };
    }

    fn persist(self) -> Result<()> {
        CurrentEscrow::update_current_escrow_data(self.inner)
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn read_intent() -> Result<Intent> {
    let mut buf = [0u8; 1];
    let mut locator = Locator::new();
    locator.pack(sfield::Memos);
    locator.pack(0);
    locator.pack(sfield::MemoData);
    let code = unsafe {
        get_tx_nested_field(
            locator.as_ptr(),
            locator.num_packed_bytes(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    if code <= 0 {
        return Err(if code == 0 {
            Error::InternalError
        } else {
            Error::from_code(code)
        });
    }
    match Intent::from_byte(buf[0]) {
        Some(intent) => Ok(intent),
        None => Err(Error::InvalidParams),
    }
}

fn identify(
    tx: AccountID,
    client: AccountID,
    freelancer: AccountID,
    arbitrator: AccountID,
) -> Option<Role> {
    if tx == client {
        Some(Role::Client)
    } else if tx == freelancer {
        Some(Role::Freelancer)
    } else if tx == arbitrator {
        Some(Role::Arbitrator)
    } else {
        None
    }
}

fn party_of(role: Role) -> Party {
    if role == Role::Client {
        Party::Client
    } else {
        Party::Freelancer
    }
}

fn deadline_release(state: &State) -> Result<bool> {
    if !state.freelancer_confirmed() {
        return Ok(false);
    }
    let mut buf = [0u8; 4];
    let code = unsafe { get_parent_ledger_time(buf.as_mut_ptr(), buf.len()) };
    if code < 0 {
        return Err(Error::from_code(code));
    }
    Ok(u32::from_le_bytes(buf) > state.deadline())
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let tx = get_current_escrow_finish();
    let tx_account = try_or_trace!(tx.get_account(), "tx_account");
    let escrow = get_current_escrow();
    let client = try_or_trace!(escrow.get_account(), "client");
    let freelancer = try_or_trace!(escrow.get_destination(), "freelancer");
    let mut state = try_or_trace!(State::load(&escrow), "state");
    let intent = try_or_trace!(read_intent(), "intent");

    let role = match identify(tx_account, client, freelancer, state.arbitrator()) {
        Some(r) => r,
        None => return 0,
    };

    // Auto-release is sticky: once the freelancer has confirmed and the deadline has
    // passed, the next EscrowFinish releases regardless of its intent. Checked here,
    // before any state mutation, so no intent (confirm, deconfirm, or dispute) can
    // clear a condition that is already met.
    if try_or_trace!(deadline_release(&state), "ledger_time") {
        return 1;
    }

    match (role, intent, state.dispute()) {
        // Participant confirm/deconfirm, no active dispute
        (
            Role::Client | Role::Freelancer,
            Intent::Confirm | Intent::Deconfirm,
            DisputeState::None,
        ) => {
            state.set_confirmation(party_of(role), intent == Intent::Confirm);
            let release = state.client_confirmed() && state.freelancer_confirmed()
                || try_or_trace!(deadline_release(&state), "ledger_time");
            try_or_trace!(state.persist(), "persist");
            release as i32
        }
        // Participant raises a dispute, clears confirmations
        (Role::Client | Role::Freelancer, Intent::Dispute, DisputeState::None) => {
            state.set_confirmation(Party::Client, false);
            state.set_confirmation(Party::Freelancer, false);
            state.set_dispute(DisputeState::ActiveBy(party_of(role)));
            try_or_trace!(state.persist(), "persist");
            0
        }
        // Disputing party withdraws their own dispute
        (Role::Client | Role::Freelancer, Intent::Undispute, DisputeState::ActiveBy(by))
            if by == party_of(role) =>
        {
            state.set_dispute(DisputeState::None);
            try_or_trace!(state.persist(), "persist");
            0
        }
        // Arbitrator rules on an active dispute
        (Role::Arbitrator, _, DisputeState::ActiveBy(_)) => match ArbRuling::from_intent(intent) {
            Some(ArbRuling::ForFreelancer) => 1,
            Some(ArbRuling::ForClient) => {
                state.set_dispute(DisputeState::ArbLocked);
                try_or_trace!(state.persist(), "persist");
                0
            }
            None => 0,
        },
        _ => 0,
    }
}
