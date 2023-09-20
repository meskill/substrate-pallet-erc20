#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

use frame_support::weights::Weight;
pub use pallet::*;
use sp_core::U256;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub trait WeightInfo {
	fn transfer() -> Weight;
	fn transfer_from() -> Weight;
	fn approve() -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
}

type Balance = U256;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, sp_runtime, sp_std::vec::Vec, DefaultNoBound};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type Name: Get<&'static str>;
		#[pallet::constant]
		type Symbol: Get<&'static str>;
		#[pallet::constant]
		type Decimals: Get<u8>;
	}

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub type TotalSupply<T: Config<I>, I: 'static = ()> =
		StorageValue<_, Balance, ValueQuery, GetDefault>;

	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type Balances<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Blake2_128, T::AccountId, Balance, ValueQuery, GetDefault>;

	#[pallet::storage]
	#[pallet::getter(fn allowance)]
	pub type Allowances<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
		_,
		Blake2_128,
		T::AccountId,
		Blake2_128,
		T::AccountId,
		Balance,
		ValueQuery,
		GetDefault,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		Transfer { from: T::AccountId, to: T::AccountId, value: Balance },
		Approval { owner: T::AccountId, spender: T::AccountId, value: Balance },
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		ERC20InsufficientBalance,
		ERC20InsufficientAllowance,
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		fn spend_allowance(
			owner: &T::AccountId,
			spender: &T::AccountId,
			value: Balance,
		) -> DispatchResult {
			let current_allowance = Allowances::<T, I>::get(&owner, &spender);

			if current_allowance < value {
				return Err(Error::<T, I>::ERC20InsufficientAllowance.into())
			}

			Allowances::<T, I>::insert(owner, spender, current_allowance - value);

			Ok(())
		}

		fn transfer_impl(from: T::AccountId, to: T::AccountId, value: Balance) -> DispatchResult {
			let from_balance = Balances::<T, I>::get(&from);

			if from_balance < value {
				return Err(Error::<T, I>::ERC20InsufficientBalance.into())
			};

			Balances::<T, I>::mutate(&from, |balance| *balance -= value);
			Balances::<T, I>::mutate(&to, |balance| *balance += value);

			Self::deposit_event(Event::Transfer { from, to, value });

			Ok(())
		}
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, value: Balance) -> DispatchResult {
			let from = ensure_signed(origin)?;

			Self::transfer_impl(from, to, value)
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::transfer_from())]
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			value: Balance,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;

			Self::spend_allowance(&from, &spender, value)?;

			Self::transfer_impl(from, to, value)
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::approve())]
		pub fn approve(
			origin: OriginFor<T>,
			spender: T::AccountId,
			value: Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;

			Allowances::<T, I>::insert(&owner, &spender, value);

			Self::deposit_event(Event::Approval { owner, spender, value });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::mint())]
		pub fn mint(origin: OriginFor<T>, account: T::AccountId, value: Balance) -> DispatchResult {
			ensure_root(origin)?;

			TotalSupply::<T, I>::mutate(|total| *total += value);
			Balances::<T, I>::mutate(&account, |balance| *balance += value);

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::burn())]
		pub fn burn(origin: OriginFor<T>, value: Balance) -> DispatchResult {
			let account = ensure_signed(origin)?;
			let balance = Balances::<T, I>::get(&account);

			if balance < value {
				return Err(Error::<T, I>::ERC20InsufficientBalance.into())
			}

			TotalSupply::<T, I>::mutate(|total| *total -= value);
			Balances::<T, I>::mutate(&account, |balance| *balance -= value);

			Ok(())
		}
	}

	#[pallet::genesis_config]
	#[derive(DefaultNoBound)]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub balances: Vec<(T::AccountId, Balance)>,
		pub _phantom_data: PhantomData<I>,
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {
			let mut total_supply: Balance = Balance::default();

			for (account, balance) in self.balances.iter() {
				Balances::<T, I>::insert(account, balance);
				total_supply += *balance;
			}

			TotalSupply::<T, I>::put(total_supply);
		}
	}
}
