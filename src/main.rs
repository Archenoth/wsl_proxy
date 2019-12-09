use regex::Regex;
use std::process::{Command, exit};

/// Returns a string representation of all the arguments passed into
/// it, wrapping things that look like pathnames with calls to wslpath
/// subshells
fn get_args(args: std::env::Args) -> String {
    return args.fold(String::from(""), |acc, next| {
        let next = next.replace("\\", "\\\\");

        if Regex::new("^[a-zA-Z]:[/\\\\]").unwrap().is_match(&next) {
            acc + " $(wslpath '" + &next + "')"
        } else {
            acc + " " + &next
        }
    })
}

fn main() {
    let mut args = std::env::args();

    let program = args.nth(0)
        .expect("Program needs to have a name!");

    let program = program.split(std::path::MAIN_SEPARATOR).last()
        .expect("Program cannot be a path seperator");

    let program = Regex::new(".[eE][xX][eE]").unwrap().split(program).next()
        .expect("Program should be called <wslprogram>.exe");

    if program == "wsl_proxy" {
        println!("Rename this program to the WSL binary you wish to call (Eg: git.exe -> git)");

        exit(1);
    } else {
        let bash_arg = program.to_owned() + &get_args(args);

        exit(Command::new("bash").arg("-lc").arg(bash_arg).status()
             .expect("Failed to run program").code()
             .expect("No exit status"));
    }
}
