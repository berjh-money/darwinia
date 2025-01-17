// --- paritytech ---
use pallet_identity::Config;
// --- darwinia-network ---
use crate::{weights::pallet_identity::WeightInfo, *};

frame_support::parameter_types! {
	// Minimum 100 bytes/CRAB deposited (1 MILLI/byte)
	pub const BasicDeposit: Balance = darwinia_deposit(1, 258);
	pub const FieldDeposit: Balance = darwinia_deposit(0, 66);
	pub const SubAccountDeposit: Balance = darwinia_deposit(1, 53);
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl Config for Runtime {
	type Event = Event;
	type Currency = Ring;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = EnsureRootOrMoreThanHalfCouncil;
	type RegistrarOrigin = EnsureRootOrMoreThanHalfCouncil;
	type WeightInfo = WeightInfo<Runtime>;
}
