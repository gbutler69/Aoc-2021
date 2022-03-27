#[test]
fn it_has_112_successful_trajectories_for_target_area_20_to_30_x_and_neg10_to_neg5_y() {
    let expected = 112;
    let actual = super::determine_all_trajectories_that_hit_target_area(20..=30, -10..=-5).len();
    assert_eq!(
        expected, actual,
        "Expected {expected} number of hit trajectories, got {actual})",
    );
}
