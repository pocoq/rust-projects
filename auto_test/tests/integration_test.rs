use auto_test as at;

#[test]
fn it_larger_can_hold_smaller() {
	let larger = at::mod_write_test::Rectangle {
		width: 8,
		height: 7,
	};
	let smaller = at::mod_write_test::Rectangle {
		width: 5,
		height: 1,
	};
	assert!(!smaller.can_hold(&larger));}
