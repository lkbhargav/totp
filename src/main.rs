use clap::{Arg, Command};
use std::error::Error;

mod accounting;
mod commands;
mod encrypt;
mod generator;
mod filehandler;

fn main() -> Result<(), Box<dyn Error>> {
    let account_arg = || {
        Arg::new("account")
            .short('a')
            .long("account")
            .value_name("account")
            .help("Generate a TOTP for a given account.")
    };
    let matches = Command::new("totp")
        .version("1.0")
        .author("Gergely Brautigam")
        .about("TOTP Token generator on the command line with AES encrypted account handling.")
        .subcommand(Command::new("add").about("Adds a new account with a TOTP token."))
        .subcommand(
            Command::new("generate")
                .about("Generate a new token for a given account.")
                .arg(account_arg()),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a given account.")
                .arg(account_arg()),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("add", _)) => commands::add_account()?,
        Some(("generate", token)) => match token.get_one::<String>("account") {
            Some(acc) => commands::generate_token(acc)?,
            None => println!("Please define an --account to generate a token for"),
        },
        Some(("delete", token)) => match token.get_one::<String>("account") {
            Some(acc) => commands::delete_account(acc)?,
            None => println!("Please define an --account to delete"),
        },
        _ => {}
    }
    Ok(())
}
