use crook_calculator::compute;

#[test]
fn compute_complex_stuff() {
    assert_eq!(compute("3 *  (  (  2 + 1   - 1 ) ^ 2 ) / 2 - 3.5 * ( 2 ) - -3.5").unwrap(), 2.5);
}

#[test]
fn compute_validation_reject() {
    assert_eq!(compute("1 + 2 2"), None);
}