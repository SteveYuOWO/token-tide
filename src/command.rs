use clap::{arg, Command};

pub fn tt_command() -> Command {
    Command::new("Token Tide")
        .about("Your swift navigator token prices.")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .about("Query list tokens by symbol or address")
                .arg(arg!(<TOKEN> "Token symbol or address"))
        )
        .subcommand(
            Command::new("query")
                .about("Query token info")
                .arg(arg!(<TOKEN> "Token symbol or address"))
        )
}