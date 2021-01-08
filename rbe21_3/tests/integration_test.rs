mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(rbe21_3::add(3,2), 5);
}