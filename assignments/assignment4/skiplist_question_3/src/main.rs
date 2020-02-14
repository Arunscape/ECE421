#![allow(dead_code)]
use std::cell::Cell;

enum Level {
    Low,
    Medium,
    High,
}
struct Task {
    id: Cell<u8>,
    level: Level,
}
fn main() {
    let task = Task {
        id: Cell::new(10),
        level: Level::High,
    };
    task.id.set(100);
    println!("Task with ID: {:?}", task.id);
}

/*
Explanation:
Mutability is a property of either a borrow (&mut) or a binding (let mut).
you cannot have a struct with some fields mutable and some immutable:

struct Point {
    x: i32,
    mut y: i32, // WRONG
}

The mutability of a struct is in its binding:
ex: let mut Point = { x: 1, y: 2 };

But, we can use Cell<T> to emulate field level mutability as seen above.
A type has interior mutability if its internal state can be changed through a
shared reference to it. Cell<T> allows this functionality, but this is done in
a single threaded way since it is not thread safe.
Cell<T> implements interior mutability by moving values in and out of the
Cell<T>. It provides methods to retrieve and change the current interior
value:

source: Rust book

*/
