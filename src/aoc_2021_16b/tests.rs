#[test]
fn it_has_a_value_of_2021_for_this_single_literal_packet() {
    let expected = 2021;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![0xD2, 0xFE, 0x28]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_3_for_this_sum_packet_of_1_and_2() {
    let expected = 3;
    let actual =
        super::determine_numeric_result_of_packet_evaluation(vec![0xC2, 0x00, 0xB4, 0x0A, 0x82]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_54_for_this_product_packet_of_6_and_9() {
    let expected = 54;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![
        0x04, 0x00, 0x5A, 0xC3, 0x38, 0x90,
    ]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_7_for_this_min_packet_of_7_8_9() {
    let expected = 7;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![
        0x88, 0x00, 0x86, 0xC3, 0xE8, 0x81, 0x12,
    ]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_9_for_this_max_packet_of_7_8_9() {
    let expected = 9;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![
        0xCE, 0x00, 0xC4, 0x3D, 0x88, 0x11, 0x20,
    ]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_1_for_this_lessthan_packet_of_5_and_15() {
    let expected = 1;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![
        0xD8, 0x00, 0x5A, 0xC2, 0xA8, 0xF0,
    ]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_0_for_this_greaterthan_packet_of_5_and_15() {
    let expected = 0;
    let actual =
        super::determine_numeric_result_of_packet_evaluation(vec![0xF6, 0x00, 0xBC, 0x2D, 0x8F]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_0_for_this_equalto_packet_of_5_and_15() {
    let expected = 0;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![
        0x9C, 0x00, 0x5A, 0xC2, 0xF8, 0xF0,
    ]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}

#[test]
fn it_has_a_value_of_1_for_this_1_plus_2_equalto_2_times_2_packet() {
    let expected = 1;
    let actual = super::determine_numeric_result_of_packet_evaluation(vec![
        0x9C, 0x01, 0x41, 0x08, 0x02, 0x50, 0x32, 0x0F, 0x18, 0x02, 0x10, 0x4A, 0x08,
    ]);
    assert_eq!(
        expected, actual,
        "Expected {expected} for literal value, got {actual}",
    );
}
