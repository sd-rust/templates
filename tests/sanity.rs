extern crate templates;

use templates::test_me;

#[test]
fn sanity_test() {
    assert_eq!("Hello world!", test_me());
}