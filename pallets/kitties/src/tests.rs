use crate::{mock::*};
use frame_support::assert_ok;
use crate::Gender;
#[test]

fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Kitties::mint(&1, [0u8; 16], Gender::Male));
		assert_ok!(Kitties::mint(&2, [1u8; 16], Gender::Female));
		System::set_block_number(2);
		let kitty_1 = Kitties::kitties_owned(1)[0];
		let kitty_2 = Kitties::kitties_owned(2)[0];
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));

		assert_ok!(Kitties::set_price(Origin::signed(1), kitty_1, Some(100)));

		assert_ok!(Kitties::buy_kitty(Origin::signed(2), kitty_1, Some(100)));

		System::set_block_number(4);
		assert_ok!(Kitties::breed_kitty(Origin::signed(2), kitty_1, kitty_2));

		assert_ok!(Kitties::transfer(Origin::signed(2), 1u64, kitty_1));
	});
}
