#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Erc20;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn transfer() {
		let caller: T::AccountId = whitelisted_caller();
		let _ = Erc20::<T>::mint(RawOrigin::Root.into(), caller.clone(), Balance::from(1000));
		let to: T::AccountId = account("to", 2, SEED);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), to.clone(), Balance::from(100));

		assert_eq!(Balances::<T>::get(&to), Balance::from(100));
	}

	#[benchmark]
	fn transfer_from() {
		let caller: T::AccountId = whitelisted_caller();
		let spender: T::AccountId = account("spender", 2, SEED);
		let to: T::AccountId = account("to", 3, SEED);

		let _ = Erc20::<T>::mint(RawOrigin::Root.into(), caller.clone(), Balance::from(1000));
		let _ = Erc20::<T>::approve(
			RawOrigin::Signed(caller.clone()).into(),
			spender.clone(),
			Balance::from(1000),
		);

		assert_eq!(Allowances::<T>::get(&caller, &spender), Balance::from(1000));

		#[extrinsic_call]
		_(RawOrigin::Signed(spender.clone()), caller.clone(), to.clone(), Balance::from(1000));

		assert_eq!(Balances::<T>::get(&caller), Balance::default());
		assert_eq!(Balances::<T>::get(&spender), Balance::default());
		assert_eq!(Balances::<T>::get(&to), Balance::from(1000));
	}

	#[benchmark]
	fn approve() {
		let caller: T::AccountId = whitelisted_caller();
		let spender: T::AccountId = account("spender", 2, SEED);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), spender.clone(), Balance::from(1000));

		assert_eq!(Allowances::<T>::get(&caller, &spender), Balance::from(1000));
	}

	#[benchmark]
	fn mint() {
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		_(RawOrigin::Root, caller.clone(), Balance::from(500));

		assert_eq!(Balances::<T>::get(&caller), Balance::from(500));
	}

	#[benchmark]
	fn burn() {
		let caller: T::AccountId = whitelisted_caller();

		let _ = Erc20::<T>::mint(RawOrigin::Root.into(), caller.clone(), Balance::from(1000));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), Balance::from(500));

		assert_eq!(Balances::<T>::get(&caller), Balance::from(500));
	}

	impl_benchmark_test_suite!(Erc20, crate::mock::new_test_ext(), crate::mock::Test);
}
