#[test]
fn the_parser_method_next_bit_u8_functions_correctly() {
    let parser = super::Parser {
        bits: vec![
            0x01, // byte 0, bit 7, caret 7
            0x02, // byte 1, bit 6, caret 14
            0x04, // byte 2, bit 5, caret 21
            0x08, // byte 3, bit 4, caret 28
            0x10, // byte 4, bit 3, caret 35
            0x20, // byte 5, bit 2, caret 42
            0x40, // byte 6, bit 1, caret 49
            0x80, // byte 7, bit 0, caret 56
        ],
    };

    let mut caret = 0;
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(1, caret);

    caret = 7;
    assert_eq!(1, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(8, caret);

    caret = 13;
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(1, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(15, caret);

    caret = 20;
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(1, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(23, caret);

    caret = 54;
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(1, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(0, parser.next_bit_u8(&mut caret).unwrap());
    assert_eq!(64, caret);

    assert_eq!(None, parser.next_bit_u8(&mut caret));
}

#[test]
fn it_has_a_version_total_of_6_for_this_single_literal_packet() {
    let expected = 6;
    let actual = super::determine_packet_and_subpacket_version_numbers(vec![0xD2, 0xFE, 0x28])
        .into_iter()
        .map(u32::from)
        .sum::<u32>();
    assert_eq!(
        expected, actual,
        "Expected {expected} for total of version numbers, got {actual}"
    );
}

#[test]
fn it_has_a_value_of_2021_for_this_single_literal_packet() {
    let expected = 2021;
    let actual = super::Parser::from(vec![0xD2, 0xFE, 0x28])
        .parse()
        .root_value();
    assert_eq!(
        Some(expected),
        actual,
        "Expected {expected} for literal value, got {actual}",
        actual = actual.unwrap()
    );
}

#[test]
fn it_has_a_version_total_of_16_for_this_operator_packet() {
    let expected = 16;
    let actual = super::determine_packet_and_subpacket_version_numbers(vec![
        0x8A, 0x00, 0x4A, 0x80, 0x1A, 0x80, 0x02, 0xF4, 0x78,
    ])
    .into_iter()
    .map(u32::from)
    .sum::<u32>();
    assert_eq!(
        expected, actual,
        "Expected {expected} for total of version numbers, got {actual}"
    );
}

#[test]
fn it_has_a_version_total_of_12_for_this_operator_packet() {
    let expected = 12;
    let actual = super::determine_packet_and_subpacket_version_numbers(vec![
        0x62, 0x00, 0x80, 0x00, 0x16, 0x11, 0x56, 0x2C, 0x88, 0x02, 0x11, 0x8E, 0x34,
    ])
    .into_iter()
    .map(u32::from)
    .sum::<u32>();
    assert_eq!(
        expected, actual,
        "Expected {expected} for total of version numbers, got {actual}"
    );
}

#[test]
fn it_has_a_version_total_of_23_for_this_operator_packet() {
    let expected = 23;
    let actual = super::determine_packet_and_subpacket_version_numbers(vec![
        0xC0, 0x01, 0x50, 0x00, 0x01, 0x61, 0x15, 0xA2, 0xE0, 0x80, 0x2F, 0x18, 0x23, 0x40,
    ])
    .into_iter()
    .map(u32::from)
    .sum::<u32>();
    assert_eq!(
        expected, actual,
        "Expected {expected} for total of version numbers, got {actual}"
    );
}

#[test]
fn it_has_a_version_total_of_31_for_this_operator_packet() {
    let expected = 31;
    let actual = super::determine_packet_and_subpacket_version_numbers(vec![
        0xA0, 0x01, 0x6C, 0x88, 0x01, 0x62, 0x01, 0x7C, 0x36, 0x86, 0xB1, 0x8A, 0x3D, 0x47, 0x80,
    ])
    .into_iter()
    .map(u32::from)
    .sum::<u32>();
    assert_eq!(
        expected, actual,
        "Expected {expected} for total of version numbers, got {actual}"
    );
}
