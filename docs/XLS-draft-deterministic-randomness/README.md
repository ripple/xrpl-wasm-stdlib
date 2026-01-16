<pre>
  title: Deterministic Randomness for Smart Escrows
  description: A standard for generating deterministic pseudo-random numbers in XRPL WASM extensions
  author: David Fuelling (@sappenin)
  status: Idea
  category: Ecosystem
  created: 2026-01-16
</pre>

# 1. Abstract

This document specifies a standard approach for generating **deterministic pseudo-random numbers** within XRPL Smart
Escrows and future WASM-based extensions. The approach uses on-chain entropy sources (parent ledger hash, escrow's
`PreviousTxnID`) combined with domain separation and counter-based expansion to produce reproducible random values that
all validators compute identically.

This is an **Ecosystem** standard that defines conventions for the `xrpl-wasm-stdlib` library. It does not require
protocol changes or amendments.

# 2. Motivation

Smart Escrows and future XRPL smart contracts often need randomness for use cases such as:

- **Fair selection**: Choosing among participants (raffles, turn order, weighted selection)
- **Randomized thresholds**: Variable delays, percentages, or limits
- **Game mechanics**: Dice rolls, card shuffles, outcome determination
- **Tie-breaking**: Deterministic resolution when multiple conditions are equal

However, true randomness is fundamentally incompatible with blockchain consensus—all validators must compute identical
results. This creates a need for **deterministic pseudo-randomness**: values that appear random and are unpredictable
before commitment, but are fully reproducible given the same inputs.

# 3. Security Model

## 3.1. Properties

| Property                             | Status        | Notes                                        |
| ------------------------------------ | ------------- | -------------------------------------------- |
| **Determinism**                      | ✅ Guaranteed | All validators compute identical results     |
| **Reproducibility**                  | ✅ Guaranteed | Same inputs always produce same outputs      |
| **Pre-commitment unpredictability**  | ⚠️ Limited    | Unpredictable until parent ledger closes     |
| **Post-commitment unpredictability** | ❌ None       | Predictable once parent ledger hash is known |
| **Liveness**                         | ✅ Guaranteed | No external parties required                 |

## 3.2. Entropy Sources

The random seed is derived from:

1. **Parent Ledger Hash** (32 bytes): Unknown until the parent ledger closes. Provides temporal unpredictability.
2. **Escrow's `PreviousTxnID`** (32 bytes): Unique per escrow instance. Provides per-escrow uniqueness.
3. **Domain Separator** (up to 64 bytes): User-provided string. Prevents cross-purpose collisions.
4. **Counter** (8 bytes): For generating multiple values from one seed.

## 3.3. Trust Assumptions

- **Validators**: Could theoretically influence results by manipulating ledger construction, though this is expensive
  and detectable on XRPL's consensus mechanism.
- **Front-running**: Anyone who observes the parent ledger hash before submitting their transaction can predict the
  random outcome for that ledger.

## 3.4. Appropriate Use Cases

✅ **Suitable for:**

- Fair selection among known participants (raffles, turn order)
- Randomized delays, thresholds, or percentages
- Game mechanics where all parties see results simultaneously
- Tie-breaking or deterministic shuffling
- Any case where the result is revealed to everyone at the same time

❌ **NOT suitable for:**

- High-value lotteries where front-running is profitable
- Generating private keys or cryptographic secrets
- Cases where one party learns the result before others can act
- Any scenario requiring true unpredictability after commitment

# 4. Specification

## 4.1. Seed Computation

The deterministic seed is computed as:

```
seed = SHA512-Half(parent_ledger_hash || previous_txn_id || domain || counter)
```

Where:

- `parent_ledger_hash`: 32-byte hash from `get_parent_ledger_hash()` host function
- `previous_txn_id`: 32-byte `PreviousTxnID` field from the current escrow ledger object
- `domain`: Up to 64 bytes of user-provided domain separator (truncated if longer)
- `counter`: 8-byte little-endian unsigned integer
- `||`: Byte concatenation
- `SHA512-Half`: The first 32 bytes of SHA-512, as used throughout XRPL

The total input buffer is fixed at 136 bytes:

- Bytes 0-31: `parent_ledger_hash`
- Bytes 32-63: `previous_txn_id`
- Bytes 64-127: `domain` (zero-padded)
- Bytes 128-135: `counter`

## 4.2. Domain Separation

Domain separators prevent accidental collisions between different uses of randomness within the same escrow or across
different escrow implementations. Developers SHOULD use unique, descriptive domain strings such as:

- `b"lottery_winner_selection_v1"`
- `b"shuffle_participants"`
- `b"random_delay_seconds"`

Domain separators are truncated to 64 bytes if longer.

## 4.3. Counter-Based Expansion

To generate multiple random values from a single seed, increment the counter for each value needed. This avoids repeated
calls to `get_parent_ledger_hash()` and `get_previous_txn_id()`.

## 4.4. Output Extraction

### 4.4.1. 64-bit Unsigned Integer

Extract the first 8 bytes of the hash as a little-endian `u64`:

```rust
let value = u64::from_le_bytes([hash[0], hash[1], ..., hash[7]]);
```

### 4.4.2. Range-Limited Values

For a value in range `[0, max)`:

```rust
let value = random_u64 % max;
```

**Note**: This has slight modulo bias for small `max` values. For most practical use cases (max > 1000), the bias is
negligible (<0.000001%). For cryptographic applications, rejection sampling should be used instead.

### 4.4.3. Boolean Values

```rust
let boolean = (random_u64 & 1) == 1;
```

### 4.4.4. Raw Bytes

The full 32-byte hash output can be used directly when raw bytes are needed.

# 5. Reference Implementation

The following Rust implementation is provided in `xrpl-wasm-stdlib`:

```rust
//! Deterministic Pseudo-Random Number Generation for XRPL Smart Escrows

use crate::core::ledger_objects::current_escrow::get_current_escrow;
use crate::core::ledger_objects::traits::CurrentEscrowFields;
use crate::core::types::uint::{Hash256, HASH256_SIZE};
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
  pub fn new(domain: &[u8]) -> Result<Self> {
    let seed = compute_seed(domain, 0)?;
    Ok(Self { seed, counter: 0 })
  }

  /// Generates the next pseudo-random u64 in the sequence.
  pub fn next_u64(&mut self) -> Result<u64> {
    self.counter = self.counter.wrapping_add(1);
    let hash = compute_seed_with_counter(&self.seed, self.counter)?;
    Ok(u64::from_le_bytes([
      hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
    ]))
  }

  /// Generates a pseudo-random value in the range `[0, max)`.
  pub fn next_range(&mut self, max: u64) -> Result<u64> {
    if max == 0 {
      return Ok(0);
    }
    let value = self.next_u64()?;
    Ok(value % max)
  }

  /// Generates 32 pseudo-random bytes.
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

/// Computes the initial seed from ledger state and domain separator.
fn compute_seed(domain: &[u8], counter: u64) -> Result<[u8; HASH256_SIZE]> {
  let mut input = [0u8; MAX_HASH_INPUT_SIZE];

  // Get parent ledger hash (bytes 0-31)
  let parent_hash = host::get_parent_ledger_hash()?;
  input[0..32].copy_from_slice(&parent_hash);

  // Get escrow's PreviousTxnID (bytes 32-63)
  let escrow = get_current_escrow()?;
  let prev_txn_id = escrow.previous_txn_id();
  input[32..64].copy_from_slice(&prev_txn_id);

  // Copy domain separator (bytes 64-127, truncated/padded)
  let domain_len = domain.len().min(MAX_DOMAIN_LEN);
  input[64..64 + domain_len].copy_from_slice(&domain[..domain_len]);

  // Add counter (bytes 128-135)
  input[128..136].copy_from_slice(&counter.to_le_bytes());

  // Hash with SHA512-Half
  host::sha512_half(&input)
}

/// Computes a new hash from seed + counter for sequence generation.
fn compute_seed_with_counter(seed: &[u8; HASH256_SIZE], counter: u64) -> Result<[u8; HASH256_SIZE]> {
  let mut input = [0u8; 40]; // 32 bytes seed + 8 bytes counter
  input[0..32].copy_from_slice(seed);
  input[32..40].copy_from_slice(&counter.to_le_bytes());
  host::sha512_half(&input)
}
```

## 5.1. Usage Examples

### 5.1.1. Simple One-Shot Random Value

```rust
use xrpl_wasm_stdlib::core::random::random_u64;

// Generate a single random value for a specific purpose
let value = random_u64(b"my_feature") ?;
```

### 5.1.2. Random Value in a Range

```rust
use xrpl_wasm_stdlib::core::random::random_range;

// Random percentage [0, 100)
let percentage = random_range(b"percentage", 100) ?;

// Random dice roll [1, 6]
let dice = random_range(b"dice_roll", 6) ? + 1;
```

### 5.1.3. Multiple Random Values

```rust
use xrpl_wasm_stdlib::core::random::DeterministicRng;

// Create an RNG for generating multiple values
let mut rng = DeterministicRng::new(b"game_logic") ?;

let roll1 = rng.next_u64() ?; let roll2 = rng.next_u64() ?; let dice = rng.next_range(6) ? + 1; // 1-6
let coin_flip = rng.next_bool() ?;
```

### 5.1.4. Fair Participant Selection

```rust
use xrpl_wasm_stdlib::core::random::DeterministicRng;

fn select_winner(participants: &[AccountId]) -> Result<&AccountId> {
  let mut rng = DeterministicRng::new(b"raffle_winner_v1")?;
  let index = rng.next_range(participants.len() as u64)? as usize;
  Ok(&participants[index])
}
```

# 6. Rationale

## 6.1. Why Not Use External Randomness?

Approaches like Verifiable Random Functions (VRFs) or commit-reveal schemes (as described
in [Pyth's secure randomness article](https://www.pyth.network/blog/secure-random-numbers-for-blockchains)) provide
stronger unpredictability guarantees but require:

- External oracle infrastructure
- Multi-round protocols with latency
- Additional trust assumptions

For many XRPL use cases, the simpler "blockhash-style" approach is sufficient and preferred due to its:

- **Zero latency**: Random values available immediately during execution
- **No external dependencies**: Uses only on-chain data
- **Guaranteed liveness**: No risk of oracle failure blocking execution

## 6.2. Why SHA512-Half?

SHA512-Half is already used throughout XRPL for object IDs, transaction hashes, and other cryptographic operations.
Using the same primitive:

- Ensures consistency with existing XRPL conventions
- Leverages existing host function implementations
- Provides 256 bits of output, sufficient for all use cases

## 6.3. Why Parent Ledger Hash + PreviousTxnID?

This combination provides:

- **Temporal unpredictability**: Parent ledger hash is unknown until the ledger closes
- **Per-escrow uniqueness**: PreviousTxnID ensures different escrows get different random streams
- **Replay protection**: The same escrow in different ledgers gets different values

## 6.4. Why Domain Separation?

Without domain separation, two different features using randomness in the same escrow would get correlated values.
Domain separation ensures:

- Independent random streams for different purposes
- No accidental collisions between libraries or features
- Clear documentation of randomness usage

## 6.5. Why Counter-Based Expansion?

Calling host functions (`get_parent_ledger_hash`, `get_previous_txn_id`) has overhead. Counter-based expansion:

- Minimizes host calls (one-time seed computation)
- Provides unlimited random values from a single seed
- Maintains determinism across all values in the sequence

# 7. Backwards Compatibility

This is a new Ecosystem standard with no backwards compatibility concerns. It defines conventions for the
`xrpl-wasm-stdlib` library that do not affect the XRPL protocol.

# 8. Security Considerations

## 8.1. Front-Running Risk

The most significant security consideration is **front-running**. Once the parent ledger hash is known (after the parent
ledger closes but before the next ledger is finalized), anyone can predict the random values for transactions in the
upcoming ledger.

**Mitigations:**

- For high-value applications, use commit-reveal schemes or external VRFs
- Design applications so that front-running provides no advantage
- Accept the risk for low-stakes applications where the cost of front-running exceeds the benefit

## 8.2. Validator Influence

Validators could theoretically influence randomness by:

- Choosing which transactions to include
- Manipulating ledger construction timing

On XRPL, this is mitigated by:

- UNL-based consensus requiring supermajority agreement
- Economic disincentives for validator misbehavior
- Detectability of manipulation attempts

## 8.3. Not Cryptographically Secure

This randomness MUST NOT be used for:

- Private key generation
- Cryptographic nonces
- Any secret that must remain unknown to validators

## 8.4. Modulo Bias

The simple `value % max` approach has slight bias for small `max` values. For `max = 3`:

- Bias is approximately `(2^64 % 3) / 2^64 ≈ 10^-19`
- This is negligible for all practical purposes

For applications requiring perfect uniformity, rejection sampling should be implemented.

# 9. Appendix

## 9.1. Comparison with Other Approaches

| Approach               | Latency   | Trust         | Liveness                  | Complexity | Front-run Resistance |
| ---------------------- | --------- | ------------- | ------------------------- | ---------- | -------------------- |
| **This Standard**      | Zero      | Validators    | Guaranteed                | Low        | Weak                 |
| **Xahau `featureRNG`** | Zero      | 5+ Validators | Requires 5+ contributions | High       | Strong               |
| Commit-Reveal          | 2+ rounds | Participants  | Requires participation    | Medium     | Strong               |
| VRF Oracles            | 1+ round  | Oracle        | Requires oracle           | High       | Strong               |
| Threshold Signatures   | 1+ round  | Committee     | Requires committee        | High       | Strong               |

## 9.2. Test Vectors

Given:

- `parent_ledger_hash`: `0x0000...0001` (32 bytes, value 1)
- `previous_txn_id`: `0x0000...0002` (32 bytes, value 2)
- `domain`: `b"test"`
- `counter`: `0`

Expected seed (SHA512-Half of concatenated input):

```
[Implementation-specific - to be computed]
```

## 9.3. FAQ

### Q: What's the difference between this and Xahau's RNG?

**A:** Xahau has implemented a protocol-level amendment called
`featureRNG` ([PR #659](https://github.com/Xahau/xahaud/pull/659)) that provides significantly stronger security
guarantees. Here's how they compare:

| Aspect                       | This Standard                            | Xahau's `featureRNG`                                     |
| ---------------------------- | ---------------------------------------- | -------------------------------------------------------- |
| **Type**                     | Ecosystem (library convention)           | Amendment (protocol change)                              |
| **Entropy Source**           | Parent ledger hash + PreviousTxnID       | Multi-validator commit-reveal + proposal signatures      |
| **Front-running Resistance** | Weak                                     | Strong                                                   |
| **Validator Collusion**      | Single source (no resistance)            | Requires 5+ validators to collude                        |
| **New Protocol Elements**    | None                                     | `ttENTROPY` tx, `ttSHUFFLE` tx, `ltRANDOM` ledger object |
| **Host Functions**           | Uses existing `get_parent_ledger_hash()` | New `dice(sides)` and `random(ptr, len)`                 |
| **Complexity**               | Simple                                   | Complex                                                  |
| **Liveness**                 | Guaranteed                               | Requires 5+ entropy contributions                        |

**Xahau's approach** uses a commit-reveal scheme where validators:

1. Commit `sha512Half(nextRandomValue)` in ledger N
2. Reveal `nextRandomValue` in ledger N+1
3. Contributions are rejected if the reveal doesn't match the commitment

Additionally, `ttSHUFFLE` pseudo-transactions shuffle the transaction ordering based on proposal signatures, preventing
ordering-based attacks.

**When to use which:**

- Use **this standard** for low-stakes applications where simplicity is preferred and front-running cost exceeds benefit
- Use **Xahau's `featureRNG`** (when available) for high-stakes applications requiring strong unpredictability
  guarantees

If `featureRNG` becomes available on your target network, prefer its `dice()` and `random()` host functions over this
library-based approach.

### Q: Can I use this for a lottery?

**A:** It depends on the stakes. For low-value lotteries where the cost of front-running (monitoring the network,
submitting competing transactions) exceeds the expected value, this approach is acceptable. For high-value lotteries,
use Xahau's `featureRNG`, commit-reveal schemes, or VRF-based approaches.

### Q: Why not use block timestamps?

**A:** Block timestamps on XRPL have limited precision and can be slightly manipulated by validators. The parent ledger
hash provides much higher entropy and is harder to influence.

### Q: Can two escrows get the same random values?

**A:** No, because each escrow has a unique `PreviousTxnID`. Even if executed in the same ledger with the same domain
separator, different escrows will produce different random values.

### Q: What if I need more than 2^64 random values?

**A:** The counter is a 64-bit integer, allowing for 2^64 values per domain. If you need more, use different domain
separators to create independent streams.

### Q: Can validators predict or manipulate the random values?

**A:** With this standard, yes—validators can predict values at execution time since they know the parent ledger hash.
They could theoretically influence results by choosing which transactions to include, though this is detectable and
economically disincentivized on XRPL.

With Xahau's `featureRNG`, manipulation requires collusion of 5+ validators who all submitted entropy contributions,
making it significantly harder.
