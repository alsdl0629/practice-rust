use practice_test::add;

#[test]
fn integration_test() {
    let result = add(2, 2);
    assert_eq!(result, 4)
}