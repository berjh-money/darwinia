// --- paritytech ---
use bp_messages::LaneId;
use bp_runtime::ChainId;
use frame_support::PalletId;
// --- darwinia-network ---
use crate::{messages::crab_message::ToCrabOutboundPayload, *};
use darwinia_bridge_primitives::{AccountIdConverter, DARWINIA_CRAB_LANE};
use darwinia_support::{evm::IntoH160, s2s::LatestMessageNoncer};
use dp_asset::{TokenMetadata, NATIVE_TOKEN_TYPE};
use to_substrate_backing::Config;

pub struct CrabMessageNoncer;
impl LatestMessageNoncer for CrabMessageNoncer {
	fn outbound_latest_generated_nonce(lane_id: LaneId) -> u64 {
		BridgeCrabMessages::outbound_latest_generated_nonce(lane_id).into()
	}

	fn inbound_latest_received_nonce(lane_id: LaneId) -> u64 {
		BridgeCrabMessages::inbound_latest_received_nonce(lane_id).into()
	}
}

frame_support::parameter_types! {
	pub const CrabChainId: ChainId = CRAB_CHAIN_ID;
	pub RingMetadata: TokenMetadata = TokenMetadata::new(
		NATIVE_TOKEN_TYPE,
		PalletId(*b"da/bring").into_h160(),
		b"Darwinia Network Native Token".to_vec(),
		b"RING".to_vec(),
		9,
	);
	pub const S2sBackingPalletId: PalletId = PalletId(*b"da/tcrbk");
	pub const MaxLockRingAmountPerTx: Balance = 100_000 * COIN;
	pub const BridgeCrabLaneId: LaneId = DARWINIA_CRAB_LANE;
}

impl Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type PalletId = S2sBackingPalletId;
	type RingMetadata = RingMetadata;
	type MaxLockRingAmountPerTx = MaxLockRingAmountPerTx;
	type RingCurrency = Ring;
	type BridgedAccountIdConverter = AccountIdConverter;
	type BridgedChainId = CrabChainId;
	type OutboundPayloadCreator = ToCrabOutboundPayload;
	type MessageNoncer = CrabMessageNoncer;
	type MessageLaneId = BridgeCrabLaneId;
	type MessagesBridge = BridgeCrabMessages;
}
