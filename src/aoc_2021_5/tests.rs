#[test]
fn test() {
    assert_eq!(
        5,
        super::determine_number_of_points_with_overlapping([
            super::Vector2D::from(((0, 9), (5, 9))),
            super::Vector2D::from(((8, 0), (0, 8))),
            super::Vector2D::from(((9, 4), (3, 4))),
            super::Vector2D::from(((2, 2), (2, 1))),
            super::Vector2D::from(((7, 0), (7, 4))),
            super::Vector2D::from(((6, 4), (2, 0))),
            super::Vector2D::from(((0, 9), (2, 9))),
            super::Vector2D::from(((3, 4), (1, 4))),
            super::Vector2D::from(((0, 0), (8, 8))),
            super::Vector2D::from(((5, 5), (8, 2))),
        ])
    );
}
