#[test]
fn test_board_from() {
    assert_eq!(
        [
            Some(22),
            Some(13),
            Some(17),
            Some(11),
            Some(0),
            Some(8),
            Some(2),
            Some(23),
            Some(4),
            Some(24),
            Some(21),
            Some(9),
            Some(14),
            Some(16),
            Some(7),
            Some(6),
            Some(10),
            Some(3),
            Some(18),
            Some(5),
            Some(1),
            Some(12),
            Some(20),
            Some(15),
            Some(19)
        ],
        super::Board::from([
            22, 13, 17, 11, 0, //
            8, 2, 23, 4, 24, //
            21, 9, 14, 16, 7, //
            6, 10, 3, 18, 5, //
            1, 12, 20, 15, 19
        ])
        .0
    )
}

#[test]
fn test() {
    assert_eq!(
        (Some(3), Some(188), Some(24), Some(4512)),
        super::determine_winning_board_uncalled_total_last_called_number_and_score(
            [
                super::Board::from([
                    22, 13, 17, 11, 0, //
                    8, 2, 23, 4, 24, //
                    21, 9, 14, 16, 7, //
                    6, 10, 3, 18, 5, //
                    1, 12, 20, 15, 19
                ]),
                super::Board::from([
                    3, 15, 0, 2, 22, //
                    9, 18, 13, 17, 5, //
                    19, 8, 7, 25, 23, //
                    20, 11, 10, 24, 4, //
                    14, 21, 16, 12, 6
                ]),
                super::Board::from([
                    14, 21, 17, 24, 4, //
                    10, 16, 15, 9, 19, //
                    18, 8, 23, 26, 20, //
                    22, 11, 13, 6, 5, //
                    2, 0, 12, 3, 7
                ]),
            ],
            [
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        )
    );
}
