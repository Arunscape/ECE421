pub const DB: &'static str = "data/users.db";
const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub struct UserBase {
    pub fname: String,
}

use bcrypt::{verify, BcryptError, DEFAULT_COST};
use chrono::NaiveDateTime;
use sqlite::Error as SqErr;

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
    OtherError(String),
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
impl From<String> for UBaseErr {
    fn from(s: String) -> Self {
        UBaseErr::OtherError(s)
    }
}

impl UserBase {
    pub fn add_user(&self, u_name: &str, p_word: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(p_word, DEFAULT_COST)?;
        let mut st = conn.prepare("insert into users(u_name, p_word, balance) values (?,?,?);")?;
        bind!(st, u_name, &hpass as &str, 0);
        st.next()?;
        Ok(())
    }
    pub fn pay(&self, u_from: &str, u_to: &str, amount: i64) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let balance = self.get_balance(u_from)?;
        if balance < amount {
            let e = format!(
                "{} has a balance of ${}, which is not enough to send ${} to {}",
                u_from, balance, amount, u_to
            );
            Err(e)
        } else {
            Ok(())
        }?;

        let mut st = conn.prepare(
            "insert into transactions (u_from, u_to, t_date,
t_amount) values(?,?,datetime(\"now\"),?);",
        )?;
        bind!(st, u_from, u_to, amount);
        st.next()?;
        self.set_balance(u_from, balance - amount)?;
        let balance = self.get_balance(u_to)?;
        self.set_balance(u_to, balance + amount)?;
        Ok(())
    }

    pub fn get_balance(&self, uname: &str) -> Result<i64, UBaseErr> {
        let conn = sqlite::open(&self.fname)?;

        let mut st = conn.prepare("select balance from users where u_name=?;")?;
        bind!(st, uname);
        st.next()?;
        let balance = st.read::<i64>(0)?;
        Ok(balance)
    }

    pub fn set_balance(&self, uname: &str, balance: i64) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;

        let mut st = conn.prepare("update users set balance=? where u_name=?;")?;
        bind!(st, balance, uname);
        st.next()?;
        Ok(())
    }

    pub fn get_transaction_history(&self, uname: &str) -> Result<(), UBaseErr> {
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
                Err("The database returned a transaction where the username you entered is not the sender or reciever".to_string())
            }?;
        }

        Ok(())
    }

    pub fn login(&self, uname: &str, password: &str) -> Result<bool, UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("select p_word from users where u_name=?;")?;
        bind!(st, uname);
        st.next()?;

        let hash = st.read::<String>(0);

        match hash {
            Ok(h) => verify(password, &h).map_err(|e| UBaseErr::from(e)),
            _ => Err(UBaseErr::OtherError("The username you entered does not exist in the database, or the password was incorrect".to_string())),
        }
    }
}

/// run with cargo test -- --nocapture --test-threads=1
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
create table users(u_name text PRIMARY KEY, p_word text, balance integer);
create table transactions(u_from text, u_to text, t_date integer, t_amount
text, PRIMARY KEY(u_from,t_date), FOREIGN KEY (u_from) REFERENCES users(u_name),
FOREIGN KEY (u_to) REFERENCES users(u_name));
insert into users (u_name, p_word, balance) values ("Matt", "matt_pw", 9000), ("Dave",
"dave_pw", 0);
insert into transactions (u_from, u_to, t_date, t_amount) values
("Dave","Matt",datetime("now"),50);"#,
                    )
                    .unwrap();
                connection
            }
        };
    }

    macro_rules! setup {
        () => {
            (db!(), UserBase { fname: DB.into() })
        };
    }
    macro_rules! assert_result_from_db {
        ($st:ident, $e:expr) => {
            if let State::Done = $st.next().unwrap() {
                panic!($e);
            };
        };
    }

    macro_rules! assert_one_result_from_db {
        ($st:ident, $e:expr) => {
            assert_result_from_db!($st, $e);
            assert_no_result_from_db!($st, "More than one transaction was found");
        };
    }
    macro_rules! assert_no_result_from_db {
        ($st:ident, $e:expr) => {
            if let State::Row = $st.next().unwrap() {
                panic!($e);
            }
        };
    }

    macro_rules! assert_balance {
        ($u:ident, $user:expr, $balance:expr) => {
            let balance = $u.get_balance($user)?;
            assert_eq!($balance, balance);
        };
    }

    use super::*;
    use chrono::Utc;
    use sqlite::State;

    #[test]
    fn add_user() -> Result<(), UBaseErr> {
        let (c, u) = setup!();
        u.add_user("new user", "new password").unwrap();

        let mut st = c.prepare("select * from users where u_name=? and balance=?")?;
        bind!(st, "new user", 0);

        assert_one_result_from_db!(st, "new user was not created in the db");
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
        assert_one_result_from_db!(st, "New transaction not found");

        // not enough to complete transaction
        assert!(u.pay("Matt", "Dave", 1).is_err());
        assert_balance!(u, "Matt", 0);
        assert_balance!(u, "Dave", 9000);
        Ok(())
    }

    #[test]
    fn get_transaction_history() -> Result<(), UBaseErr> {
        /* Run with cargo test --nocapture --test-threads=1
         * Output should be
         test test::get_transaction_history ... Dave sent $50 to Matt on YYYY-MM-DD HH:MM am/pm
         Dave sent $50 to Matt on 2020-03-12 07:12 pm
         Dave received $9000 from Matt on 2020-03-12 07:12 pm
         ok
        */
        let (_, u) = setup!();
        u.get_transaction_history("Dave")?;
        u.pay("Matt", "Dave", 9000)?;
        u.get_transaction_history("Dave")?;
        Ok(())
    }

    #[test]
    fn get_balance() -> Result<(), UBaseErr> {
        let (_, u) = setup!();
        assert_balance!(u, "Matt", 9000);
        assert_balance!(u, "Dave", 0);

        // not enough for transaction
        assert!(u.pay("Dave", "Matt", 50).is_err());
        assert_balance!(u, "Matt", 9000);
        assert_balance!(u, "Dave", 0);

        u.pay("Matt", "Dave", 50)?;
        assert_balance!(u, "Matt", 9000 - 50);
        assert_balance!(u, "Dave", 50);

        Ok(())
    }
    #[test]
    fn set_balance() -> Result<(), UBaseErr> {
        let (_, u) = setup!();
        assert_balance!(u, "Matt", 9000);
        assert_balance!(u, "Dave", 0);

        u.set_balance("Matt", 12)?;
        assert_balance!(u, "Matt", 12);
        u.set_balance("Dave", 13)?;
        assert_balance!(u, "Dave", 13);

        Ok(())
    }
}
