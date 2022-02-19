#[test]
fn test() {
    assert_eq!(
        1134,
        super::determine_area_of_3_largest_basins_rank(
            super::HeightMap::default()
                .add_row(vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0])
                .add_row(vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1])
                .add_row(vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2])
                .add_row(vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9])
                .add_row(vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8])
        )
    );
}
