#[test]
fn it_flashes_all_octopi_at_step_195() {
    let total_flashes = super::determine_step_where_all_octopi_flash(
        [
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ],
        1000,
    );
    assert_eq!(195, total_flashes.unwrap());
}
