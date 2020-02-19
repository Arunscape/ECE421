fn main() {
    println!("Hello, world!");
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
