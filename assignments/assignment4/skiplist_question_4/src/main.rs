mod linkedlist;
mod skiplist;

use crate::skiplist::SkipList;

fn main() {
    let mut list: SkipList<usize> = SkipList::new();

    println!("{:?}", list);

    list.push(30);
    println!("{:?}", list);
}

