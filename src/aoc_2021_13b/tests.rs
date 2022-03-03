#[test]
fn it_forms_a_capital_o_after_the_given_folds() {
    let output = super::determine_points_after_folds(
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
        [(super::Axis::Row, 7), (super::Axis::Column, 5)],
    );
    assert_eq!(
        vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', ' ', ' ', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#'],
            vec!['#', '#', '#', '#', '#'],
        ],
        output
    );
}
