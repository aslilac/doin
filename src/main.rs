mod completions;

use std::{env, io, process::Command};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let dir_or_flag = args.next().expect("Must provide a directory to run in!");

    if dir_or_flag == "--fish-completions" {
        println!("{}", completions::FISH);
        return Ok(());
    }

    let dir = dir_or_flag;
    let command = args.next().expect("Must provide a command to run!");
    Command::new(command).current_dir(dir).args(args).status()?;
    Ok(())
}
