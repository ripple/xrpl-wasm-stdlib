// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.

use crate::host::Result;
use crate::objects::traits::CurrentLedgerObjectCommonFields;
use crate::objects::traits::LedgerObjectCommonFields;
use crate::objects::{current_ledger_object, ledger_object};
use crate::sfield;
use crate::types::account_id::AccountID;
use crate::types::amount::Amount;
use crate::types::blob::PublicKeyBlob;
use crate::types::uint::Hash256;

/// Trait providing access to fields specific to PayChannel objects in any ledger.
pub trait PayChannelFields: LedgerObjectCommonFields {
    /// The source address that owns this payment channel. This comes from the sending address of the transaction that created the channel.
    fn account(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Account)
    }

    /// The destination address for this payment channel. While the payment channel is open, this address is the only one that can receive XRP from the channel. This comes from the `Destination` field of the transaction that created the channel.
    fn destination(&self) -> Result<AccountID> {
        ledger_object::get_field(self.get_slot_num(), sfield::Destination)
    }

    /// The Sequence field (Optional).
    fn sequence(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Sequence)
    }

    /// Total [XRP, in drops][], that have been allocated to this channel. This includes amounts that have been paid to the destination address. This is initially set by the transaction that created the channel and can be increased if the source address sends a `PaymentChannelFund` transaction.
    fn amount(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Amount)
    }

    /// Total [XRP, in drops][] already paid out by the channel. The difference between this value and the `Amount` field is how much can still be paid to the destination address with `PaymentChannelClaim` transactions. If the channel closes, the remaining difference is returned to the source address.
    fn balance(&self) -> Result<Amount> {
        ledger_object::get_field(self.get_slot_num(), sfield::Balance)
    }

    /// Public key, in hexadecimal, of the key pair that can be used to sign claims against this channel. This can be any valid secp256k1 or Ed25519 public key. This is set by the transaction that created the channel and must match the public key used in claims against the channel. The channel source address can also send XRP from this channel to the destination without signed claims.
    fn public_key(&self) -> Result<PublicKeyBlob> {
        ledger_object::get_field(self.get_slot_num(), sfield::PublicKey)
    }

    /// Number of seconds the source address must wait to close the channel if it still has any XRP in it. Smaller values mean that the destination address has less time to redeem any outstanding claims after the source address requests to close the channel. Can be any value that fits in a 32-bit unsigned integer (0 to 2^32-1). This is set by the transaction that creates the channel.
    fn settle_delay(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::SettleDelay)
    }

    /// The mutable expiration time for this payment channel, in [seconds since the Ripple Epoch][]. The channel is expired if this value is present and smaller than the previous ledger's `close_time` field. See Channel Expiration for more details.
    fn expiration(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::Expiration)
    }

    /// The immutable expiration time for this payment channel, in [seconds since the Ripple Epoch][]. This channel is expired if this value is present and smaller than the previous ledger's `close_time` field. This is optionally set by the transaction that created the channel, and cannot be changed.
    fn cancel_after(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::CancelAfter)
    }

    /// An arbitrary tag to further specify the source for this payment channel, such as a hosted recipient at the owner's address.
    fn source_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this payment channel, such as a hosted recipient at the destination address.
    fn destination_tag(&self) -> Result<Option<u32>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationTag)
    }

    /// A hint indicating which page of the source address's owner directory links to this entry, in case the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        ledger_object::get_field(self.get_slot_num(), sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        ledger_object::get_field(self.get_slot_num(), sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the destination's owner directory links to this entry, in case the directory consists of multiple pages. Omitted on payment channels created before enabling the [fixPayChanRecipientOwnerDir amendment][].
    fn destination_node(&self) -> Result<Option<u64>> {
        ledger_object::get_field_optional(self.get_slot_num(), sfield::DestinationNode)
    }
}

/// Trait providing access to fields specific to the current PayChannel object.
pub trait CurrentPayChannelFields: CurrentLedgerObjectCommonFields {
    /// The source address that owns this payment channel. This comes from the sending address of the transaction that created the channel.
    fn account(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Account)
    }

    /// The destination address for this payment channel. While the payment channel is open, this address is the only one that can receive XRP from the channel. This comes from the `Destination` field of the transaction that created the channel.
    fn destination(&self) -> Result<AccountID> {
        current_ledger_object::get_field(sfield::Destination)
    }

    /// The Sequence field (Optional).
    fn sequence(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Sequence)
    }

    /// Total [XRP, in drops][], that have been allocated to this channel. This includes amounts that have been paid to the destination address. This is initially set by the transaction that created the channel and can be increased if the source address sends a `PaymentChannelFund` transaction.
    fn amount(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Amount)
    }

    /// Total [XRP, in drops][] already paid out by the channel. The difference between this value and the `Amount` field is how much can still be paid to the destination address with `PaymentChannelClaim` transactions. If the channel closes, the remaining difference is returned to the source address.
    fn balance(&self) -> Result<Amount> {
        current_ledger_object::get_field(sfield::Balance)
    }

    /// Public key, in hexadecimal, of the key pair that can be used to sign claims against this channel. This can be any valid secp256k1 or Ed25519 public key. This is set by the transaction that created the channel and must match the public key used in claims against the channel. The channel source address can also send XRP from this channel to the destination without signed claims.
    fn public_key(&self) -> Result<PublicKeyBlob> {
        current_ledger_object::get_field(sfield::PublicKey)
    }

    /// Number of seconds the source address must wait to close the channel if it still has any XRP in it. Smaller values mean that the destination address has less time to redeem any outstanding claims after the source address requests to close the channel. Can be any value that fits in a 32-bit unsigned integer (0 to 2^32-1). This is set by the transaction that creates the channel.
    fn settle_delay(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::SettleDelay)
    }

    /// The mutable expiration time for this payment channel, in [seconds since the Ripple Epoch][]. The channel is expired if this value is present and smaller than the previous ledger's `close_time` field. See Channel Expiration for more details.
    fn expiration(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::Expiration)
    }

    /// The immutable expiration time for this payment channel, in [seconds since the Ripple Epoch][]. This channel is expired if this value is present and smaller than the previous ledger's `close_time` field. This is optionally set by the transaction that created the channel, and cannot be changed.
    fn cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::CancelAfter)
    }

    /// An arbitrary tag to further specify the source for this payment channel, such as a hosted recipient at the owner's address.
    fn source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::SourceTag)
    }

    /// An arbitrary tag to further specify the destination for this payment channel, such as a hosted recipient at the destination address.
    fn destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_field_optional(sfield::DestinationTag)
    }

    /// A hint indicating which page of the source address's owner directory links to this entry, in case the directory consists of multiple pages.
    fn owner_node(&self) -> Result<u64> {
        current_ledger_object::get_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_field(sfield::PreviousTxnID)
    }

    /// The [index of the ledger][Ledger Index] that contains the transaction that most recently modified this entry.
    fn previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_field(sfield::PreviousTxnLgrSeq)
    }

    /// A hint indicating which page of the destination's owner directory links to this entry, in case the directory consists of multiple pages. Omitted on payment channels created before enabling the [fixPayChanRecipientOwnerDir amendment][].
    fn destination_node(&self) -> Result<Option<u64>> {
        current_ledger_object::get_field_optional(sfield::DestinationNode)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PayChannel {
    pub(crate) slot_num: i32,
}

impl PayChannel {
    /// Binds this handle to a host-managed slot holding a PayChannel ledger object.
    pub fn new(slot_num: i32) -> Self {
        Self { slot_num }
    }
}

impl LedgerObjectCommonFields for PayChannel {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl PayChannelFields for PayChannel {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::host_bindings_trait::MockHostBindings;
    use crate::host::setup_mock;
    use crate::objects::test_utils::*;

    #[test]
    fn read_all_fields() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_present(&mut mock);
        let _guard = setup_mock(mock);

        let obj = PayChannel::new(0);

        assert!(obj.account().is_ok());
        assert!(obj.destination().is_ok());
        assert!(obj.amount().is_ok());
        assert!(obj.balance().is_ok());
        assert!(obj.public_key().is_ok());
        assert!(obj.settle_delay().is_ok());
        assert!(obj.owner_node().is_ok());
        assert!(obj.previous_txn_id().is_ok());
        assert!(obj.previous_txn_lgr_seq().is_ok());
        assert!(obj.sequence().is_ok());
        assert!(obj.expiration().is_ok());
        assert!(obj.cancel_after().is_ok());
        assert!(obj.source_tag().is_ok());
        assert!(obj.destination_tag().is_ok());
        assert!(obj.destination_node().is_ok());
    }

    #[test]
    fn optional_fields_none() {
        let mut mock = MockHostBindings::new();
        mock_all_fields_not_found(&mut mock);
        let _guard = setup_mock(mock);

        let obj = PayChannel::new(0);

        assert!(obj.sequence().unwrap().is_none());
        assert!(obj.expiration().unwrap().is_none());
        assert!(obj.cancel_after().unwrap().is_none());
        assert!(obj.source_tag().unwrap().is_none());
        assert!(obj.destination_tag().unwrap().is_none());
        assert!(obj.destination_node().unwrap().is_none());
    }
}
