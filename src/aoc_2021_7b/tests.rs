#[test]
fn test() {
    assert_eq!(
        (5, 168),
        super::determine_position_and_minimum_fuel_required(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
    );
}
