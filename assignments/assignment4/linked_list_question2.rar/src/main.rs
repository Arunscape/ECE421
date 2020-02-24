use im::list::{cons, List};
use std::fmt;

// makes testing more expressive
// allows the creation of a linked list
// ex: linkedlist!{1=>2=>3=>4=>5}
#[macro_export]
macro_rules! linkedlist {
    ( $x:expr ) => {
        LinkedList::new($x)
    };
    ( $head:expr $( => $rest:expr )* ) => {
        linkedlist!($($rest)=>*).push($head)
    };
}

fn main() {
    let list = linkedlist! {2=>3=>5=>7};
    println!("For example, if we have a list as follows:");
    println!("{}", list);
    let list = list.push(1);
    println!(".push(1) should result in the following list:");
    println!("{}", list);

    println!();

    println!("Similarly, if we have a list as follows:");
    let list = linkedlist! {2=>3=>5=>7};
    println!("{}", list);
    let list = list.push_back(1);
    println!(".push_back(1) should result in the following list:");
    println!("{}", list);
}

#[derive(Debug, PartialEq)]
struct LinkedList<T> {
    list: List<T>,
}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Self { list: List::new() }
    }

    pub fn new(t: T) -> Self {
        Self {
            list: List::new().cons(t),
        }
    }

    pub fn push(self, t: T) -> Self {
        Self {
            list: self.list.cons(t),
        }
    }

    pub fn push_back(self, t: T) -> Self {
        // snoc is cons spelled backwards
        Self {
            list: self.list.snoc(t),
        }
    }
}

// so we can print the linked list in this format
// 1=>2=>3=>4=>5
impl<T> fmt::Display for LinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.list.head().unwrap());
        for thing in self.list.iter().skip(1) {
            write!(f, "=>{}", thing).unwrap();
        }
        Ok(())
    }
}

// from: https://docs.rs/im/5.0.0/src/im/list.rs.html#83
/*
The words `car` and `cdr` come from Lisp, and were the original
names of the functions to get the left and the right hands of a
cons cell, respectively. Cons cells in Lisp were simply containers
for two values: the car and the cdr (pronounced 'cudder'), and,
Lisp being an untyped language, had no restrictions on cons cells
forming proper lists, but this is how they were most commonly used:
forming singly linked lists by having the left hand side contain a
value, and the right hand side a pointer to the rest of the list.

`cons` is short for 'construct', which is the easy one. `car` means
'contents of address register' and `cdr` means 'contents of decrement
register.' These were the registers on the CPU of the IBM 704 computer
(on which Lisp was originally implemented) used to hold the respective
values.

Lisp also commonly provided pre-composed sequences of the `car` and
`cdr` functions, such as `cadr`, the `car` of the `cdr`, ie. the
second element of a list, and `cddr`, the list with the two first
elements dropped. Pronunciation goes like this: `cadr` is, obviously,
'cadder', while `cddr` is 'cududder', and `caddr` (the `car` of the
`cdr` of the `cdr`) is 'cadudder'. It can get a little subtle for the
untrained ear.
*/

