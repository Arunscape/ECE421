mod userbase;

use rpassword;
use userbase::{UBaseErr, UserBase, DB};

#[derive(Debug)]
enum Error {
    ParseNumber(&'static str),
    UBaseErr(UBaseErr),
    WrongPassword(&'static str),
}

impl From<UBaseErr> for Error {
    fn from(s: UBaseErr) -> Self {
        Error::UBaseErr(s)
    }
}

fn main() -> Result<(), Error> {
    use clap::{load_yaml, App};
    let yml = load_yaml!("cli.yml");
    let matches = App::from(yml).get_matches();

    let u = UserBase { fname: DB.into() };

    match matches.subcommand() {
        ("new", Some(n)) => {
            // these are required arguments, so they should never panic
            let username = n.value_of("username").expect("Could not parse username");
            let password = n.value_of("password").expect("Could not parse password");
            println!("Creating user...");
            u.add_user(username, password)?;
        }
        ("transfer", Some(t)) => {
            let from = t.value_of("from").expect("Could not parse sender username");
            let to = t.value_of("to").expect("Could not parse reciever username");
            let amount = t
                .value_of("amount")
                .expect("Could not parse transaction amount");
            let amount: i64 = amount.parse().map_err(|_| Error::ParseNumber("Failed to parse the transaction amount as a number. Input entered could not be converted to a 64-bit integer"))?;

            let password =
                rpassword::read_password_from_tty(Some("Please input your password: ")).unwrap();

            println!("Verifying password...");

            let valid = u.login(from, &password)?;

            match valid {
                true => {
                    println!("Sending...");
                    u.pay(from, to, amount).map_err(|e| Error::from(e))
                }
                false => Err(Error::WrongPassword(
                    "Wrong password!, Aborting Transaction...",
                )),
            }?;

            println!(
                "Successful transaction. {} sent ${} to {}.",
                from, amount, to
            );
        }
        ("balance", Some(b)) => {
            let username = b.value_of("username").expect("Could not parse username");
            let password =
                rpassword::read_password_from_tty(Some("Please input your password: ")).unwrap();

            println!("Verifying password...");

            let valid = u.login(username, &password)?;

            match valid {
                true => {
                    let balance = u.get_balance(username)?;
                    println!("Your balance is ${}", balance);
                    Ok(())
                }
                false => Err(Error::WrongPassword("Wrong password!, Aborting...")),
            }?;
        }
        ("history", Some(b)) => {
            let username = b.value_of("username").expect("Could not parse username");
            let password =
                rpassword::read_password_from_tty(Some("Please input your password: ")).unwrap();

            println!("Verifying password...");

            let valid = u.login(username, &password)?;

            match valid {
                true => u
                    .get_transaction_history(username)
                    .map_err(|e| Error::from(e)),
                false => Err(Error::WrongPassword("Wrong password!, Aborting...")),
            }?;
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
    Ok(())
}
