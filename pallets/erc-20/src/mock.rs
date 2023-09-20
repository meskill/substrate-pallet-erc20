use crate::{self as pallet_erc_20, Balance};
use frame_support::{traits::{ConstU16, ConstU64}, instances::Instance1};
use sp_core::{ConstU8, Get, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Erc20: pallet_erc_20,
		Erc20AnotherInstance: pallet_erc_20::<Instance1>,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

macro_rules! str_getter {
	($id: ident, $val: expr) => {
		pub struct $id;

		impl Get<&'static str> for $id {
			fn get() -> &'static str {
				$val
			}
		}
	};
}

str_getter!(Erc20Name, "token name");
str_getter!(Erc20Symbol, "TS");

impl pallet_erc_20::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_erc_20::weights::WeightInfo<Test>;
	type Name = Erc20Name;
	type Symbol = Erc20Symbol;
	type Decimals = ConstU8<18>;
}

str_getter!(Erc20Name1, "another token");
str_getter!(Erc20Symbol1, "RS");

impl pallet_erc_20::Config<Instance1> for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_erc_20::weights::WeightInfo<Test>;
	type Name = Erc20Name1;
	type Symbol = Erc20Symbol1;
	type Decimals = ConstU8<18>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let initial_balance = Balance::from(1000);

	pallet_erc_20::GenesisConfig::<Test> {
		balances: (1..5).into_iter().map(|id| (id, initial_balance)).collect(),
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();


	let initial_balance = Balance::from(500);

	pallet_erc_20::GenesisConfig::<Test, Instance1> {
		balances: (1..5).into_iter().map(|id| (id, initial_balance)).collect(),
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}
