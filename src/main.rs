use std::{env, io, process::Command};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let dir = args.next().expect("Must provide a directory to run in!");
    let command = args.next().expect("Must provide a command to run!");

    Command::new(command).current_dir(dir).args(args).status()?;

    Ok(())
}
