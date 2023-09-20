use crate::{mock::*, Balance, Error, Event};
use frame_support::{assert_noop, assert_ok, instances::Instance1};
use sp_runtime::traits::BadOrigin;

#[test]
fn transfer_success() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(Erc20::balance_of(1), Balance::from(1000));
		assert_eq!(Erc20::balance_of(2), Balance::from(1000));
		assert_ok!(Erc20::transfer(RuntimeOrigin::signed(1), 2, Balance::from(100)));
		assert_eq!(Erc20::balance_of(1), Balance::from(900));
		assert_eq!(Erc20::balance_of(2), Balance::from(1100));

		System::assert_last_event(
			Event::Transfer::<Test, ()> { from: 1, to: 2, value: Balance::from(100) }.into(),
		);
	});
}

#[test]
fn transfer_failed_insufficient_balance() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::balance_of(1), Balance::from(1000));
		assert_eq!(Erc20::balance_of(2), Balance::from(1000));
		assert_noop!(
			Erc20::transfer(RuntimeOrigin::signed(1), 2, Balance::from(2000)),
			Error::<Test>::ERC20InsufficientBalance
		);
		assert_eq!(Erc20::balance_of(1), Balance::from(1000));
		assert_eq!(Erc20::balance_of(2), Balance::from(1000));
	});
}

#[test]
fn transfer_failed_unsigned_transaction() {
	new_test_ext().execute_with(|| {
		assert_noop!(Erc20::transfer(RuntimeOrigin::none(), 2, Balance::from(100)), BadOrigin);
	});
}

#[test]
fn approve_success() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(Erc20::allowance(1, 2), Balance::default());
		assert_ok!(Erc20::approve(RuntimeOrigin::signed(1), 2, Balance::from(100)));
		assert_eq!(Erc20::allowance(1, 2), Balance::from(100));
		System::assert_last_event(
			Event::Approval::<Test, ()> { owner: 1, spender: 2, value: Balance::from(100) }.into(),
		);
	});
}

#[test]
fn spend_allowance_success() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Erc20::approve(RuntimeOrigin::signed(1), 2, Balance::from(100)));
		assert_ok!(Erc20::transfer_from(RuntimeOrigin::signed(2), 1, 3, Balance::from(20)));
		System::assert_last_event(
			Event::Transfer::<Test, ()> { from: 1, to: 3, value: Balance::from(20) }.into(),
		);
		assert_ok!(Erc20::transfer_from(RuntimeOrigin::signed(2), 1, 4, Balance::from(50)));
		System::assert_last_event(
			Event::Transfer::<Test, ()> { from: 1, to: 4, value: Balance::from(50) }.into(),
		);

		assert_eq!(Erc20::balance_of(1), Balance::from(930));
		assert_eq!(Erc20::balance_of(2), Balance::from(1000));
		assert_eq!(Erc20::balance_of(3), Balance::from(1020));
		assert_eq!(Erc20::balance_of(4), Balance::from(1050));
	});
}

#[test]
fn spend_allowance_failure_insufficient_allowance() {
	new_test_ext().execute_with(|| {
		assert_ok!(Erc20::approve(RuntimeOrigin::signed(1), 2, Balance::from(100)));
		assert_ok!(Erc20::transfer_from(RuntimeOrigin::signed(2), 1, 3, Balance::from(100)));

		assert_noop!(
			Erc20::transfer_from(RuntimeOrigin::signed(2), 1, 4, Balance::from(10)),
			Error::<Test>::ERC20InsufficientAllowance
		);
	});
}

#[test]
fn spend_allowance_failure_insufficient_funds() {
	new_test_ext().execute_with(|| {
		assert_ok!(Erc20::approve(RuntimeOrigin::signed(1), 2, Balance::from(1500)));

		assert_noop!(
			Erc20::transfer_from(RuntimeOrigin::signed(2), 1, 3, Balance::from(1200)),
			Error::<Test>::ERC20InsufficientBalance
		);
	});
}

#[test]
fn mint_new_tokens() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::total_supply(), Balance::from(4000));
		assert_ok!(Erc20::mint(RuntimeOrigin::root(), 2, Balance::from(500)));
		assert_eq!(Erc20::total_supply(), Balance::from(4500));
		assert_eq!(Erc20::balance_of(2), Balance::from(1500));
	});
}

#[test]
fn burn_tokens() {
	new_test_ext().execute_with(|| {
		assert_eq!(Erc20::total_supply(), Balance::from(4000));
		assert_ok!(Erc20::burn(RuntimeOrigin::signed(2), Balance::from(500)));
		assert_eq!(Erc20::total_supply(), Balance::from(3500));
		assert_eq!(Erc20::balance_of(2), Balance::from(500));
	});
}

#[test]
fn different_instances_are_independent() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(Erc20::balance_of(1), Balance::from(1000));
		assert_eq!(Erc20::balance_of(2), Balance::from(1000));
		assert_eq!(Erc20AnotherInstance::balance_of(1), Balance::from(500));
		assert_eq!(Erc20AnotherInstance::balance_of(2), Balance::from(500));

		assert_ok!(Erc20::transfer(RuntimeOrigin::signed(1), 2, Balance::from(100)));
		assert_eq!(Erc20::balance_of(1), Balance::from(900));
		assert_eq!(Erc20::balance_of(2), Balance::from(1100));
		assert_eq!(Erc20AnotherInstance::balance_of(1), Balance::from(500));
		assert_eq!(Erc20AnotherInstance::balance_of(2), Balance::from(500));

		System::assert_last_event(
			Event::Transfer::<Test, ()> { from: 1, to: 2, value: Balance::from(100) }.into(),
		);

		assert_ok!(Erc20AnotherInstance::transfer(RuntimeOrigin::signed(1), 2, Balance::from(50)));
		assert_eq!(Erc20::balance_of(1), Balance::from(900));
		assert_eq!(Erc20::balance_of(2), Balance::from(1100));
		assert_eq!(Erc20AnotherInstance::balance_of(1), Balance::from(450));
		assert_eq!(Erc20AnotherInstance::balance_of(2), Balance::from(550));

		System::assert_last_event(
			Event::Transfer::<Test, Instance1> { from: 1, to: 2, value: Balance::from(50) }.into(),
		);
	});
}
