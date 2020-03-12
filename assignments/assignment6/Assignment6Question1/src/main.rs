const DB: &'static str = "data/users.db";
const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub struct UserBase {
    fname: String,
}

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::NaiveDateTime;
use sqlite::Error as SqErr;
use std::error::Error;

macro_rules! bind {
    // Empty case to end the recursion
    ($st:ident, $n:expr ;) => {};
    // Match the current count, the current type, and whatever else comes after
    ($st:ident, $n:expr ; $x:expr $(, $rest:expr)*) => {
        $st.bind($n, $x)?;
        // Recurse, incrementing counter and only passing the remaining params
        bind!($st, $n + 1; $($rest),*);
    };
    ($st:ident, $($xx:expr),+) => { bind!($st, 1; $($xx),*); };
}

#[derive(Debug)]
pub enum UBaseErr {
    DbErr(SqErr),
    HashError(BcryptError),
    TimeParseError(chrono::ParseError),
    OtherError(&'static str),
}

impl From<SqErr> for UBaseErr {
    fn from(s: SqErr) -> Self {
        UBaseErr::DbErr(s)
    }
}
impl From<BcryptError> for UBaseErr {
    fn from(b: BcryptError) -> Self {
        UBaseErr::HashError(b)
    }
}
impl From<chrono::ParseError> for UBaseErr {
    fn from(s: chrono::ParseError) -> Self {
        UBaseErr::TimeParseError(s)
    }
}
impl From<&'static str> for UBaseErr {
    fn from(s: &'static str) -> Self {
        UBaseErr::OtherError(s)
    }
}

impl UserBase {
    pub fn add_user(&self, u_name: &str, p_word: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(p_word, DEFAULT_COST)?;
        let mut st = conn.prepare("insert into users(u_name, p_word) values (?,?);")?;
        bind!(st, u_name, &hpass as &str);
        st.next()?;
        Ok(())
    }
    pub fn pay(&self, u_from: &str, u_to: &str, amount: i64) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(
            "insert into transactions (u_from, u_to, t_date,
t_amount) values(?,?,datetime(\"now\"),?);",
        )?;
        bind!(st, u_from, u_to, amount);
        st.next()?;
        Ok(())
    }

    fn get_transaction_history(&self, uname: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(
            "select u_from, u_to, t_date, t_amount from transactions where u_from=? or u_to=?;",
        )?;

        bind!(st, uname, uname);

        while let sqlite::State::Row = st.next()? {
            let from = st.read::<String>(0)?;
            let to = st.read::<String>(1)?;
            let date = st.read::<String>(2)?;
            let date = NaiveDateTime::parse_from_str(&date, DATE_FORMAT)?;
            let date = date.format("%Y-%m-%d %I:%M %P");
            let amount = st.read::<String>(3)?;

            if uname == to {
                println!("{} received ${} from {} on {}", to, amount, from, date);
                Ok(())
            } else if uname == from {
                println!("{} sent ${} to {} on {}", from, amount, to, date);
                Ok(())
            } else {
                Err("The database returned a transaction where the username you entered is not the sender or reciever")
            }?;
        }

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    macro_rules! db {
        // `()` indicates that the macro takes no argument.
        () => {
            // The macro will expand into the contents of this block.
            {
                let connection = sqlite::open(DB).unwrap();
                connection
                    .execute(
                        r#"
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS transactions;
create table users(u_name text PRIMARY KEY, p_word text);
create table transactions(u_from text, u_to text, t_date integer, t_amount
text, PRIMARY KEY(u_from,t_date), FOREIGN KEY (u_from) REFERENCES users(u_name),
FOREIGN KEY (u_to) REFERENCES users(u_name));
insert into users (u_name, p_word) values ("Matt", "matt_pw"), ("Dave",
"dave_pw");
insert into transactions (u_from, u_to, t_date, t_amount) values
("Dave","Matt",datetime("now"),50);"#,
                    )
                    .unwrap();
                connection
            }
        };
    }
    macro_rules! userbase {
        () => {
            UserBase { fname: DB.into() }
        };
    }

    macro_rules! setup {
        () => {
            (db!(), userbase!())
        };
    }
    macro_rules! assert_result_from_db {
        ($st:ident, $e:expr) => {
            if let State::Done = $st.next().unwrap() {
                panic!($e);
            };
        };
    }

    use super::*;
    use chrono::offset::Local;
    use chrono::DateTime;
    use chrono::Utc;
    use sqlite::State;

    #[test]
    fn add_user() -> Result<(), UBaseErr> {
        let (c, u) = setup!();
        u.add_user("new user", "new password").unwrap();

        let mut st = c.prepare("select * from users where u_name=?")?;
        bind!(st, "new user");

        assert_result_from_db!(st, "new user was not created in the db");
        Ok(())
    }

    #[test]
    fn pay() -> Result<(), UBaseErr> {
        let (c, u) = setup!();
        u.pay("Matt", "Dave", 9000)?;

        let mut st = c.prepare(
            "select * from transactions where u_from=? and u_to=? and t_date=? and t_amount=?",
        )?;
        let date: &str = &Utc::now().format(DATE_FORMAT).to_string();

        bind!(st, "Matt", "Dave", date, 9000);

        u.get_transaction_history("Matt");
        assert_result_from_db!(st, "new transaction not found");
        Ok(())
    }

    #[test]
    fn get_transaction_history() -> Result<(), UBaseErr> {
        let (c, u) = setup!();
        u.get_transaction_history("Dave")?;
        u.pay("Matt", "Dave", 9000);
        u.get_transaction_history("Dave")?;
        Ok(())
    }
}
