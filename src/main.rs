mod dexscreener;
mod command;
mod number;
mod constants;

#[tokio::main]
async fn main()  {
    let matches = command::tt_command().get_matches();
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            let token = sub_matches.get_one::<String>("TOKEN").expect("required");
            println!("Searching {} ...", token.to_uppercase());
            dexscreener::search(token).await
        },
        Some(("query", sub_matches)) => {
            let token = sub_matches.get_one::<String>("TOKEN").expect("required");
            println!("Searching {} ...", token.to_uppercase());
            dexscreener::query(token).await
        }
        _ => unreachable!(),
    }
}