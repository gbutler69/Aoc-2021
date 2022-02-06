use super::Command::*;

#[test]
fn test() {
    assert_eq!(
        (10, 15),
        super::determine_depth_and_forward_movement_amount([
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2)
        ])
    );
}
