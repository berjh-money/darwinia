pub use pallet_bridge_grandpa::Instance1 as WithDarwiniaGrandpa;

// --- paritytech ---
use pallet_bridge_grandpa::Config;
// --- darwinia-network ---
use crate::*;

frame_support::parameter_types! {
	// This is a pretty unscientific cap.
	//
	// Note that once this is hit the pallet will essentially throttle incoming requests down to one
	// call per block.
	pub const MaxRequests: u32 = 50;
	// Number of headers to keep.
	//
	// Assuming the worst case of every header being finalized, we will keep headers for at least a
	// week.
	pub const HeadersToKeep: u32 = 7 * DAYS as u32;
}

impl Config<WithDarwiniaGrandpa> for Runtime {
	type BridgedChain = Darwinia;
	type MaxRequests = MaxRequests;
	type HeadersToKeep = HeadersToKeep;
	type WeightInfo = ();
}
