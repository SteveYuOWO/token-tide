#![allow(dead_code)]
mod command;
mod config;
mod constants;
mod dexscreener;
mod number;

#[tokio::main]
async fn main() {
    let _ = config::Config::load();
    let matches = command::tt_command().get_matches();
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            let token = sub_matches.get_one::<String>("TOKEN").expect("required");
            println!("Searching {} ...", token.to_uppercase());
            dexscreener::search(token).await
        }
        Some(("query", sub_matches)) => {
            let simple = sub_matches.get_one::<bool>("simple").unwrap_or(&false);
            let token = sub_matches.get_one::<String>("TOKEN").expect("required");
            println!("Searching {} ...", token.to_uppercase());
            dexscreener::query(token, &simple).await
        }
        _ => unreachable!(),
    }
}
