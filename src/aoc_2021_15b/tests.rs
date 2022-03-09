#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_83() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1], //
            vec![1, 3], // Expanded to 5x5 tiles
                        // 1, 1, 2, 2, 3, 3, 4, 4, 5, 5
                        // 1, 3, 2, 4, 3, 5, 4, 6, 5, 7

                        // 2, 2, 3, 3, 4, 4, 5, 5, 6, 6
                        // 2, 4, 3, 5, 4, 6, 5, 7, 6, 8

                        // 3, 3, 4, 4, 5, 5, 6, 6, 7, 7
                        // 3, 5, 4, 6, 5, 7, 6, 8, 7, 9

                        // 4, 4, 5, 5, 6, 6, 7, 7, 8, 8
                        // 4, 6, 5, 7, 6, 8, 7, 9, 8, 1

                        // 5, 5, 6, 6, 7, 7, 8, 8, 9, 9
                        // 5, 7, 6, 8, 7, 9, 8, 1, 9, 2
        ]);
    assert_eq!(83, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_100() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 6],
            vec![1, 3, 8],
            vec![2, 1, 3],
        ]);
    assert_eq!(100, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_124() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 6, 3],
            vec![1, 3, 8, 1],
            vec![2, 1, 3, 6],
        ]);
    assert_eq!(124, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_134() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 6, 3],
            vec![1, 3, 8, 1],
            vec![2, 1, 3, 6],
            vec![3, 6, 9, 4],
        ]);
    assert_eq!(134, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_175() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 1, 1],
            vec![9, 9, 9, 1],
            vec![1, 1, 1, 1],
            vec![1, 9, 9, 9],
            vec![1, 1, 1, 4],
        ]);
    assert_eq!(175, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_151() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 6, 3, 7],
            vec![1, 3, 8, 1, 3],
            vec![2, 1, 3, 6, 5],
            vec![3, 6, 9, 4, 9],
        ]);
    assert_eq!(151, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_148() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 6, 3],
            vec![1, 3, 8, 1],
            vec![2, 1, 3, 6],
            vec![3, 6, 9, 4],
            vec![7, 4, 6, 3],
        ]);
    assert_eq!(148, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_158() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 6, 3, 7],
            vec![1, 3, 8, 1, 3],
            vec![2, 1, 3, 6, 5],
            vec![3, 6, 9, 4, 9],
            vec![7, 4, 6, 3, 4],
        ]);
    assert_eq!(158, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_173_also() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
            vec![1, 1, 5, 1, 1],
            vec![9, 9, 4, 9, 9],
            vec![5, 1, 9, 1, 1],
            vec![1, 9, 2, 1, 9],
            vec![1, 1, 9, 2, 4],
        ]);
    assert_eq!(173, danger_cost);
}

#[test]
fn it_has_a_path_from_start_to_end_that_has_the_lowest_danger_of_315() {
    let danger_cost =
        super::determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(vec![
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
    assert_eq!(315, danger_cost);
}
