use crate::{mock::*, Error};
use frame_support::{assert_ok, assert_err, assert_noop, error::BadOrigin};

#[test]
fn request_to_join_club() {
    new_test_ext().execute_with(|| {
		//Anyone can submit a request to root waiting list
		assert_ok!(ClubPalletTesting::request_to_join_club(Origin::signed(1)));
		// Submitted trnx should return a Ok
		assert_eq!(ClubPalletTesting::request_to_join_club(Origin::signed(2)),  Ok(()));
    })
}



#[test]
fn remove_member_from_club(){
    new_test_ext().execute_with(|| {
		// Not accessible by a normal user
		assert_noop!(ClubPalletTesting::remove_member_from_club(Origin::signed(1),100), BadOrigin);
	
    })
}

#[test]
fn add_requested_member_to_club() {
    new_test_ext().execute_with(|| {

		// trying to add a value , which is not in the root request list
        assert_err!(
            ClubPalletTesting::add_requested_member_to_club(Origin::root(), 51), 
			Error::<Test>::NotMember
        );
    })
}