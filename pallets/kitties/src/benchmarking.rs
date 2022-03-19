//! Benchmarking setup for pallet-template
 #![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	set_price{ 
		let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
	}: _(RawOrigin::Signed(caller), kitty_hashes[0], Some(s.into()))

	transfer{
		let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
		let to: T::AccountId = account("to", 2u32, 2u32);
		let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
	} : _(RawOrigin::Signed(caller), to, kitty_hashes[0])

	// buy_kitty{
	// 	let s = 0;
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let _ = Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
	// 	let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
	// 	let _ = Kitties::<T>::set_price(RawOrigin::Signed(caller).into(), kitty_hashes[0], Some(s.into()));
	// 	let buyer: T::AccountId = account("buyer", 2u32, 2u32);
	// }: _(RawOrigin::Signed(caller), kitty_hashes[0], Some(s.into()))

	breed_kitty{
		let s in 0 .. 10000000;
		let caller: T::AccountId = whitelisted_caller();
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into());
		let kitty_hashes = Kitties::<T>::kitties_owned(caller.clone());
	}: _(RawOrigin::Signed(caller), kitty_hashes[0], kitty_hashes[1])

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
