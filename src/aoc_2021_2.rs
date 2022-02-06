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

pub fn command_from_text(textual_command: String) -> Command {
    let mut fields = textual_command.split_ascii_whitespace();
    let command = fields.next().unwrap();
    let amount = fields.next().unwrap().parse::<u32>().unwrap();
    match command {
        "forward" => Command::Forward(amount),
        "up" => Command::Up(amount),
        "down" => Command::Down(amount),
        _ => panic!("Unrecognized command"),
    }
}

pub enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}
