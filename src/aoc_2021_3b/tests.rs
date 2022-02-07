#[test]
fn test() {
    assert_eq!(
        (23, 10, 230),
        super::determine_oxygen_co2_and_life_support_ratings([
            0b00100.into(),
            0b11110.into(),
            0b10110.into(),
            0b10111.into(),
            0b10101.into(),
            0b01111.into(),
            0b00111.into(),
            0b11100.into(),
            0b10000.into(),
            0b11001.into(),
            0b00010.into(),
            0b01010.into(),
        ])
    );
}
