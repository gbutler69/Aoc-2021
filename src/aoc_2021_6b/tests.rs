#[test]
fn test_18_days() {
    assert_eq!(
        26,
        super::determine_number_of_fish_after_x_days(vec![3, 4, 3, 1, 2], 18)
    );
}

#[test]
fn test_80_days() {
    assert_eq!(
        5934,
        super::determine_number_of_fish_after_x_days(vec![3, 4, 3, 1, 2], 80)
    );
}

#[test]
fn test_256_days() {
    assert_eq!(
        26_984_457_539,
        super::determine_number_of_fish_after_x_days(vec![3, 4, 3, 1, 2], 256)
    );
}
