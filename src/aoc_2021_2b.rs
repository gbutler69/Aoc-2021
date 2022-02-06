use crate::aoc_2021_2::Command;

#[cfg(test)]
mod tests;

pub fn determine_depth_and_forward_movement_amount(
    commands: impl IntoIterator<Item = Command>,
) -> (u32, u32) {
    let (mut depth, mut forward, mut heading) = (0, 0, 0);
    for command in commands {
        match command {
            Command::Up(amount) => heading -= amount,
            Command::Down(amount) => heading += amount,
            Command::Forward(amount) => {
                forward += amount;
                depth += amount * heading;
            }
        }
    }
    (depth, forward)
}
