#[test]
fn it_has_17_points_after_a_single_fold_along_row_7() {
    let total_points = super::determine_number_of_points_after_folds(
        [
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ],
        [(super::Axis::Row, 7)],
    );
    assert_eq!(17, total_points);
}
