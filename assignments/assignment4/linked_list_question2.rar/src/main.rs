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

/*EXPLANATION OF THE CONS FUNCTION
 * Cons is short for the work 'construct'.
 * This function is used in many Lisp dialects, its purpose is to take two items
 * and return a pair of those items. Pairs can also be contained in other pairs.
 * This allows for lists to be easily created using the following syntax (or something similar):
 * (cons 1 (cons 2 (cons 3 nil)))
 * for the im library, the cons function takes in two items, and returns a List
 * struct, as defined by the library itself.
 * using this implementation in rust, a list can easily be made using the
 * following syntax:
 * cons(1, cons(2, cons(3, List::new())))
*/

// A more in-depth explanation is found below:
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn macro_test() {
        assert_eq!(
            LinkedList {
                list: cons(1, cons(2, cons(3, cons(5, cons(7, List::new()))))),
            },
            linkedlist! {1=>2=>3=>5=>7}
        );
    }

    #[test]
    pub fn push() {
        let list = linkedlist! {2=>3=>5=>7};
        let list = list.push(1);
        assert_eq!(linkedlist! {1=>2=>3=>5=>7}, list);
    }

    #[test]
    pub fn push_back() {
        let list = linkedlist! {2=>3=>5=>7};
        let list = list.push_back(1);
        assert_eq!(linkedlist! {2=>3=>5=>7=>1}, list);
    }

    #[test]
    fn it_works() {
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(
            l,
            LinkedList {
                list: cons(4, cons(3, List::new())),
            }
        );

        l = l.push_back(2);
        assert_eq!(
            l,
            LinkedList {
                list: cons(4, cons(3, cons(2, List::new()))),
            }
        );
    }
}
