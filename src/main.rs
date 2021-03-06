extern crate base64;

use base64::encode;
use regex::Regex;
use std::process::{exit, Command};

/// Returns a string representation of all the arguments passed into it, wrapping things that look
/// like pathnames with calls to wslpath subshells
///
/// # Arguments
///
/// * `args` - The arguments to string-escape, and wrap in wslpath subshells when applicable
fn get_args(args: std::env::Args) -> String {
    let windrive_regex = Regex::new(r"^[a-zA-Z]:[/\\]").unwrap();
    let escape_regex = Regex::new(r#"([\\'])"#).unwrap();

    args.fold(String::from(""), |acc, next| {
        let escaped = "$'".to_owned() + &escape_regex.replace_all(&next, r"\$1") + "'";

        if windrive_regex.is_match(&next) {
            acc + r#" "$(wslpath "# + &escaped + r#")""#
        } else {
            acc + " " + &escaped
        }
    })
}

/// Verifies that the passed in program name is valid, and returns a Result containing a version
/// that can be passed into bash, or a user-facing error message in the event that the input is
/// invalid for some reason
///
/// # Arguments
///
/// * `program` - A String representation of the current binary's name
fn get_program_name(program: &str) -> Result<&str, &str> {
    let program = match program.split(std::path::MAIN_SEPARATOR).last() {
        Some(program) => program,
        None => return Err("Program name cannot be a path seperator, or blank"),
    };

    let program = match Regex::new(".[eE][xX][eE]").unwrap().split(program).next() {
        Some(program) => program,
        None => return Err("Program should be called <wslprogram>.exe"),
    };

    match program {
        "wsl_proxy" => Err("Rename this file to the WSL program to call (Eg: git.exe -> git)"),
        "bash" => Err("wsl_proxy renamed to 'bash'; exiting so we don't call ourself recursively"),
        _ => Ok(program),
    }
}

/// We base64 everything after escaping it because escaping things for Windows is a minor nightmare
fn main() {
    let mut args = std::env::args();
    let program = args.nth(0).expect("Program needs to have a name!");

    let exit_status = match get_program_name(&program) {
        Ok(program) => {
            let command = program.to_owned() + &get_args(args);
            Command::new("bash")
                .arg("-ic")
                .arg(format!("$(echo {} | base64 --decode)", encode(&command)))
                .status()
                .expect("Failed to run program")
                .code()
                .expect("No exit status")
        }
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    };

    exit(exit_status)
}
