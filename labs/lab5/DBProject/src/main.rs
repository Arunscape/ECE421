const DB: &'static str = "data/users.db";

pub struct UserBase {
    fname: String,
}

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use sqlite::Error as SqErr;
#[derive(Debug)]
pub enum UBaseErr {
    DbErr(SqErr),
    HashError(BcryptError),
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

impl UserBase {
    pub fn add_user(&self, u_name: &str, p_word: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(p_word, DEFAULT_COST)?;
        let mut st = conn.prepare("insert into users(u_name, p_word) values (?,?);")?;
        st.bind(1, u_name)?;
        st.bind(2, &hpass as &str)?;
        st.next()?;
        Ok(())
    }
    pub fn pay(&self, u_from: &str, u_to: &str, amount: i64) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(
            "insert into transactions (u_from, u_to, t_date,
t_amount) values(?,?,datetime(\"now\"),?);",
        )?;
        st.bind(1, u_from)?;
        st.bind(2, u_to)?;
        st.bind(3, amount)?;
        st.next()?;
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
    use super::*;
    use chrono::Utc;
    use sqlite::State;

    #[test]
    fn add_user() {
        let (c, u) = setup!();
        u.add_user("new user", "new password").unwrap();

        let mut statement = c.prepare("select * from users where u_name=?").unwrap();
        statement.bind(1, "new user").unwrap();

        if let State::Done = statement.next().unwrap() {
            panic!("new user was not created in the db");
        }
    }

    #[test]
    fn pay() {
        let (c, u) = setup!();
        u.pay("Matt", "Dave", 9000);

        let mut statement = c
            .prepare(
                "select * from transactions where u_from=? and u_to=? and t_date=? and t_amount=?",
            )
            .unwrap();
        let date: &str = &Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        statement.bind(1, "Matt").unwrap();
        statement.bind(2, "Dave").unwrap();
        statement.bind(3, date).unwrap();
        statement.bind(4, 9000).unwrap();

        if let State::Done = statement.next().unwrap() {
            panic!("new transaction not found");
        }
    }
}
