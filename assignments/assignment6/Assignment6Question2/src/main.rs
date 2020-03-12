#[derive(Debug)]
enum Error {
    ParseNumber(&'static str),
}

fn main() -> Result<(), Error> {
    use clap::{load_yaml, App};
    let yml = load_yaml!("cli.yml");
    let matches = App::from(yml).get_matches();

    match matches.subcommand() {
        ("new", Some(n)) => {
            // these are required arguments, so they should never panic
            let username = n.value_of("username").expect("Could not parse username");
            let password = n.value_of("password").expect("Could not parse password");
            println!("username: {}, pw: {}", username, password);
        }
        ("transfer", Some(t)) => {
            let from = t.value_of("from").expect("Could not parse sender username");
            let to = t.value_of("to").expect("Could not parse reciever username");
            let amount = t
                .value_of("amount")
                .expect("Could not parse transaction amount");
            let amount: f64 = amount.parse().map_err(|_| Error::ParseNumber("Failed to parse the transaction amount as a number. Input entered could not be converted to a 64-bit floating point number"))?;
            println!("from: {}, to: {}, amount: {}", from, to, amount);
        }
        ("balance", Some(b)) => {
            let username = b.value_of("username").expect("Could not parse username");
            println!("username: {}", username);
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
    Ok(())
}

