#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_4() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1], //
        vec![1, 3],
    ]);
    assert_eq!(4, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_7() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6],
        vec![1, 3, 8],
        vec![2, 1, 3],
    ]);
    assert_eq!(7, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_13() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6, 3],
        vec![1, 3, 8, 1],
        vec![2, 1, 3, 6],
    ]);
    assert_eq!(13, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_17() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6, 3],
        vec![1, 3, 8, 1],
        vec![2, 1, 3, 6],
        vec![3, 6, 9, 4],
    ]);
    assert_eq!(17, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_16() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 1, 1],
        vec![9, 9, 9, 1],
        vec![1, 1, 1, 1],
        vec![1, 9, 9, 9],
        vec![1, 1, 1, 4],
    ]);
    assert_eq!(16, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_26() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6, 3, 7],
        vec![1, 3, 8, 1, 3],
        vec![2, 1, 3, 6, 5],
        vec![3, 6, 9, 4, 9],
    ]);
    assert_eq!(26, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_20() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6, 3],
        vec![1, 3, 8, 1],
        vec![2, 1, 3, 6],
        vec![3, 6, 9, 4],
        vec![7, 4, 6, 3],
    ]);
    assert_eq!(20, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_24() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6, 3, 7],
        vec![1, 3, 8, 1, 3],
        vec![2, 1, 3, 6, 5],
        vec![3, 6, 9, 4, 9],
        vec![7, 4, 6, 3, 4],
    ]);
    assert_eq!(24, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_24_also() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 5, 1, 1],
        vec![9, 9, 4, 9, 9],
        vec![5, 1, 9, 1, 1],
        vec![1, 9, 2, 1, 9],
        vec![1, 1, 9, 2, 4],
    ]);
    assert_eq!(24, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_40() {
    let danger_cost = super::determine_danger_cost_of_least_cost_path(vec![
        vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
        vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
        vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
        vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
        vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
        vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
        vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
        vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
        vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
        vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
    ]);
    assert_eq!(40, danger_cost);
}
