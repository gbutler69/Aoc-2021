#[test]
fn it_has_a_best_trajectory_of_6_9_for_target_area_20_to_30_x_and_neg10_to_neg5_y() {
    let expected = (45, (6, 9));
    let actual =
        super::determine_best_trajectory_for_highest_reach_that_hits_target_area(20..=30, -10..=-5);
    assert_eq!(
        expected, actual,
        "Expected {expected_height} - ({expected_x},{expected_y}) for trajectory with max height, got {actual_height} - ({actual_x},{actual_y})",
        expected_height = expected.0,
        expected_x = expected.1.0,
        expected_y = expected.1.1,
        actual_height = actual.0,
        actual_x = actual.1.0,
        actual_y = actual.1.1
    );
}
