mod completions;

use std::{
    env,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let dir_or_flag = args.next().expect("Must provide a directory to run in!");

    if dir_or_flag == "--fish-completions" {
        println!("{}", completions::FISH);
        return ExitCode::SUCCESS;
    }

    let dir = dir_or_flag;
    let command = args.next().expect("Must provide a command to run!");
    let status = Command::new(command)
        .current_dir(dir)
        .args(args)
        .status()
        .expect("Failed to execute the command!");

    match status.code() {
        Some(0) => ExitCode::SUCCESS,
        Some(code) => ExitCode::from(code as u8),
        None => ExitCode::FAILURE,
    }
}
