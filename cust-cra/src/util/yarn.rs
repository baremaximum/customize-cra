use std::process::{Command, ExitStatus};

pub fn run_yarn_commands(mut args: Vec<&str>) -> ExitStatus {
    // takes an array of arguments and passes them to yarn.
    // returns the process status object.
    let mut commands = vec!["/c", "yarn"];
    commands.append(&mut args);
    Command::new("cmd").args(commands).status().unwrap()
}
