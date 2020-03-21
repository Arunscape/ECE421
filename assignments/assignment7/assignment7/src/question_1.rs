use std::sync::{Arc, Mutex};

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
        let from = match self.accounts.lock().as_ref() {
            Ok(s) => Ok(s),
            Err(_) => Err(()),
        }?;
        let to = match self.accounts.lock().as_ref() {
            Ok(t) => Ok(t),
            Err(_) => Err(()),
        }?;
        println!("from {}", from);
        println!("to {}", to);
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

pub fn main() {}
