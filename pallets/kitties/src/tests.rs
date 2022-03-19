use crate::{mock::*, Error};
use frame_support::assert_ok;
use crate::Gender;
#[test]

fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let mut kitty_1 = Kitties::generate_kitty(Origin::signed(1));
		System::set_block_number(2);
		let mut kitty_2 = Kitties::generate_kitty(Origin::signed(2));
		System::set_block_number(3);
		assert_ok!(Kitties::create_kitty(Origin::signed(1)));

		assert_ok!(Kitties::set_price(Origin::signed(1), kitty_1.dna, Some(0)));

		let old_owner = kitty_1.owner;
		kitty_1.gender = Gender::Male;
		kitty_2.gender = Gender::Female;

		assert_ok!(Kitties::buy_kitty(Origin::signed(2), kitty_1.dna, Some(0)));

		System::set_block_number(4);
		assert_ok!(Kitties::breed_kitty(Origin::signed(2), kitty_1.dna, kitty_2.dna));

		assert_ok!(Kitties::transfer(Origin::signed(2), old_owner, kitty_1.dna));
	});
}
