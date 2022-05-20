use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_creates_a_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1); // events are emitted from block 1

		assert_eq!(0, KittiesPallet::next_kitty_id());
		assert_ok!(KittiesPallet::create(Origin::signed(1)));
		assert_eq!(1, KittiesPallet::next_kitty_id());
	});
}
