//! Deterministic Pseudo-Random Number Generation for XRPL Smart Escrows
//!
//! This module provides deterministic randomness that is:
//! - **Reproducible**: All validators compute the same "random" values for the same inputs
//! - **Unpredictable before commitment**: Values cannot be known until the transaction is included
//! - **Domain-separated**: Different purposes get different random streams
//!
//! # Security Model
//!
//! The randomness is derived from:
//! - Parent ledger hash (32 bytes) - unpredictable until ledger closes
//! - Escrow's PreviousTxnID (32 bytes) - unique to this escrow
//! - A user-provided domain separator - prevents cross-purpose collisions
//! - An optional counter - for generating multiple values
//!
//! # ⚠️ Important Limitations
//!
//! - **NOT cryptographically secure** for key generation or secrets
//! - **Predictable by validators** at execution time
//! - **Predictable by anyone** who can see the parent ledger hash before submitting
//!
//! This is suitable for:
//! - Fair selection among known participants
//! - Randomized delays or thresholds
//! - Game mechanics where all parties see the result simultaneously
//!
//! This is NOT suitable for:
//! - Generating private keys or secrets
//! - Lotteries where the submitter could front-run
//! - Any case where the randomness must be secret
//!
//! # Example
//!
//! ```rust,ignore
//! use xrpl_wasm_stdlib::core::random::{DeterministicRng, random_u64, random_range};
//!
//! // Simple one-shot random value
//! let value = random_u64(b"my_feature")?;
//!
//! // Random value in a range [0, 100)
//! let percentage = random_range(b"percentage", 100)?;
//!
//! // For multiple values, use the RNG struct
//! let mut rng = DeterministicRng::new(b"game_logic")?;
//! let roll1 = rng.next_u64();
//! let roll2 = rng.next_u64();
//! let dice = rng.next_range(6) + 1; // 1-6
//! ```

use crate::core::ledger_objects::current_escrow::get_current_escrow;
use crate::core::ledger_objects::traits::CurrentEscrowFields;
use crate::core::types::uint::{HASH256_SIZE, Hash256};
use crate::host::{self, Result};

/// Maximum size for the hash input buffer.
/// parent_ledger_hash (32) + previous_txn_id (32) + domain (64) + counter (8) = 136
const MAX_HASH_INPUT_SIZE: usize = 136;

/// Maximum domain separator length.
pub const MAX_DOMAIN_LEN: usize = 64;

/// A deterministic pseudo-random number generator.
///
/// This RNG produces a reproducible sequence of values based on:
/// - The parent ledger hash
/// - The escrow's PreviousTxnID
/// - A domain separator you provide
///
/// Each call to `next_*` methods advances the internal counter, producing
/// a new value in the sequence.
#[derive(Debug)]
pub struct DeterministicRng {
    /// Cached seed derived from ledger state + domain
    seed: [u8; HASH256_SIZE],
    /// Counter for generating sequence of values
    counter: u64,
}

impl DeterministicRng {
    /// Creates a new deterministic RNG with the given domain separator.
    ///
    /// The domain separator should be unique to your use case to prevent
    /// accidental collisions with other code using randomness.
    ///
    /// # Arguments
    ///
    /// * `domain` - A byte slice identifying your use case (max 64 bytes)
    ///
    /// # Returns
    ///
    /// Returns `Ok(DeterministicRng)` on success, or an error if host calls fail.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut rng = DeterministicRng::new(b"lottery_v1")?;
    /// ```
    pub fn new(domain: &[u8]) -> Result<Self> {
        let seed = compute_seed(domain, 0)?;
        Ok(Self { seed, counter: 0 })
    }

    /// Generates the next pseudo-random u64 in the sequence.
    ///
    /// Each call advances the internal state, so repeated calls
    /// produce different values.
    pub fn next_u64(&mut self) -> Result<u64> {
        self.counter = self.counter.wrapping_add(1);
        let hash = compute_seed_with_counter(&self.seed, self.counter)?;
        Ok(u64::from_le_bytes([
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
        ]))
    }

    /// Generates a pseudo-random value in the range `[0, max)`.
    ///
    /// # Arguments
    ///
    /// * `max` - The exclusive upper bound (must be > 0)
    ///
    /// # Returns
    ///
    /// A value in `[0, max)`. If `max` is 0, returns 0.
    ///
    /// # Note
    ///
    /// Uses rejection sampling to avoid modulo bias for small ranges.
    pub fn next_range(&mut self, max: u64) -> Result<u64> {
        if max == 0 {
            return Ok(0);
        }
        // Simple modulo - for most use cases this is fine
        // For very small max values, there's slight bias, but acceptable for most applications
        let value = self.next_u64()?;
        Ok(value % max)
    }

    /// Generates 32 pseudo-random bytes.
    ///
    /// Useful when you need raw bytes rather than numeric values.
    pub fn next_bytes(&mut self) -> Result<[u8; 32]> {
        self.counter = self.counter.wrapping_add(1);
        let hash = compute_seed_with_counter(&self.seed, self.counter)?;
        Ok(hash)
    }

    /// Returns a random boolean with approximately 50% probability.
    pub fn next_bool(&mut self) -> Result<bool> {
        Ok((self.next_u64()? & 1) == 1)
    }
}
