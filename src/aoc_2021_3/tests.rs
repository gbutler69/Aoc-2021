#[test]
fn test() {
    assert_eq!(
        (22, 9, 198),
        super::determine_gamma_epsilon_and_power_factors([
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
