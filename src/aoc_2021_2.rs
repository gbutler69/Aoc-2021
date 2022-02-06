#[cfg(test)]
mod tests;

pub fn determine_depth_and_forward_movement_amount(
    commands: impl IntoIterator<Item = Command>,
) -> (u32, u32) {
    let (mut depth, mut forward) = (0, 0);
    for command in commands {
        match command {
            Command::Up(amount) => depth -= amount,
            Command::Down(amount) => depth += amount,
            Command::Forward(amount) => forward += amount,
        }
    }
    (depth, forward)
}

pub enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}
