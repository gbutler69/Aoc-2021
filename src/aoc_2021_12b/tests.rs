#[test]
fn it_has_36_paths_through_this_map() {
    let total_paths = super::determine_number_of_paths_from_start_to_end([
        ("start".into(), "A".into()),
        ("start".into(), "b".into()),
        ("A".into(), "c".into()),
        ("A".into(), "b".into()),
        ("b".into(), "d".into()),
        ("A".into(), "end".into()),
        ("b".into(), "end".into()),
    ]);
    assert_eq!(36, total_paths);
}

#[test]
fn it_has_103_paths_through_this_map() {
    let total_paths = super::determine_number_of_paths_from_start_to_end([
        ("dc".into(), "end".into()),
        ("HN".into(), "start".into()),
        ("start".into(), "kj".into()),
        ("dc".into(), "start".into()),
        ("dc".into(), "HN".into()),
        ("LN".into(), "dc".into()),
        ("HN".into(), "end".into()),
        ("kj".into(), "sa".into()),
        ("kj".into(), "HN".into()),
        ("kj".into(), "dc".into()),
    ]);
    assert_eq!(103, total_paths);
}

#[test]
fn it_has_3509_paths_through_this_map() {
    let total_paths = super::determine_number_of_paths_from_start_to_end([
        ("fs".into(), "end".into()),
        ("he".into(), "DX".into()),
        ("fs".into(), "he".into()),
        ("start".into(), "DX".into()),
        ("pj".into(), "DX".into()),
        ("end".into(), "zg".into()),
        ("zg".into(), "sl".into()),
        ("zg".into(), "pj".into()),
        ("pj".into(), "he".into()),
        ("RW".into(), "he".into()),
        ("fs".into(), "DX".into()),
        ("pj".into(), "RW".into()),
        ("zg".into(), "RW".into()),
        ("start".into(), "pj".into()),
        ("he".into(), "WI".into()),
        ("zg".into(), "he".into()),
        ("pj".into(), "fs".into()),
        ("start".into(), "RW".into()),
    ]);
    assert_eq!(3509, total_paths);
}
