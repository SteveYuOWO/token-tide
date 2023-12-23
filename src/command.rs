use clap::{arg, Command};

pub fn tt_command() -> Command {
    Command::new("Token Tide")
        .about("Your swift navigator token prices.")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .arg(arg!(<TOKEN> "Query list tokens by symbol or address"))
                .arg(arg!(--simple <TOKEN> "Query token price by symbol or address"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("query")
                .arg(arg!(<TOKEN> "Query token info"))
                .arg(arg!(--simple "Query token price by symbol or address"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .arg(arg!(<ADDRESS> "Priority query address(When searching for related tokens)")),
        )
}
