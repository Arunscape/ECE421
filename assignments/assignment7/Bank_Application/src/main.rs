use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone, Debug)]
struct Bank {
    accounts: Arc<Mutex<Vec<i32>>>,
}

impl Bank {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(0);
        }
        Bank {
            accounts: Arc::new(Mutex::new(v)),
        }
    }
    fn transfer(&self, from: usize, to: usize, amount: i32) -> Result<(), ()> {
        let accounts = self.accounts.lock();
        let mut accounts = match accounts {
            Ok(a) => Ok(a),
            Err(_) => Err(()),
        }?;

        // if either of the accounts from or to does not exist, an error will
        // be returned instead
        let from_amount = match accounts.get(from) {
            Some(f) => Ok(f),
            None => Err(()),
        }?;

        let _to_amount = match accounts.get(to) {
            Some(t) => Ok(t),
            None => Err(()),
        }?;

        if *from_amount < amount {
            eprintln!(
                "Account id: {} has ${} which is less than ${}. Aborting transaction",
                from, from_amount, amount
            );
            return Err(());
        };

        // println!("before\n{:?}", accounts);
        accounts[from] -= amount;
        accounts[to] += amount;
        // println!("after\n{:?}", accounts);
        println!(
            "Amount of ${} transferred from account id: {} to account id: {}",
            amount, from, to
        );
        Ok(())
    }
}
struct Person {
    ac_id: usize,
    buddy_id: usize,
}
impl Person {
    pub fn new(id: usize, b_id: usize) -> Self {
        Person {
            ac_id: id,
            buddy_id: b_id,
        }
    }
}

fn do_transaction(bank: &Bank, account: usize) -> thread::JoinHandle<()> {
    let bank = bank.clone();
    thread::spawn(move || {
        let buddy = (account + 1) % 10;
        let amount = account as i32 + 1;
        let person = Person::new(account, buddy);
        if bank
            .transfer(person.ac_id, person.buddy_id, amount)
            .is_err()
        {
            eprintln!("Error: one of the accounts don't exist");
        };
    })
}

fn main() -> Result<(), ()> {
    let bank = Bank {
        accounts: Arc::new(Mutex::new(vec![0, 2, 4, 6, 8, 10, 12, 13, 14, 16])),
    };

    let mut thread_handles = Vec::new();

    for i in 0..10 {
        thread_handles.push(do_transaction(&bank, i));
    }
    // nonexistent account
    thread_handles.push(do_transaction(&bank, 10));

    for h in thread_handles {
        h.join().unwrap();
    }

    Ok(())
}
