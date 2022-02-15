#[test]
fn test_18_days() {
    assert_eq!(
        26,
        super::determine_number_of_fish_after_x_days(
            vec![
                super::Fish::from(3),
                super::Fish::from(4),
                super::Fish::from(3),
                super::Fish::from(1),
                super::Fish::from(2)
            ],
            18
        )
    );
}

#[test]
fn test_80_days() {
    assert_eq!(
        5934,
        super::determine_number_of_fish_after_x_days(
            vec![
                super::Fish::from(3),
                super::Fish::from(4),
                super::Fish::from(3),
                super::Fish::from(1),
                super::Fish::from(2)
            ],
            80
        )
    );
}
