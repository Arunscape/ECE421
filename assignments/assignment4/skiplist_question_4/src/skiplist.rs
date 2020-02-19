use crate::linkedlist::LinkedList;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct SkipList<T> {
    //add your code here
    length: usize,
    level: usize,
    list: LinkedList<Vec<LinkedList<T>>>,
    //    random_generator: Cell<ThreadRng>,
}

impl<T> SkipList<T> {
    fn new() -> Self {
        // creates a new skip list.
        SkipList {
            length: 0,
            level: 0,
            list: LinkedList::empty(),
            //          random_generator: Cell::new(thread_rng()),
        }
    }

    // returns the number of elements at level 0 of the skip list.
    fn len(&self) -> usize {
        self.length
    }

    // checks if the skip list is empty.
    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn push(&mut self, value: T) {
        // add an element with value T to the front of the skiplist.
        self.length += 1;

        let random_level = Self::random_level();

        if self.level < random_level {
            self.level = random_level;
        }
        l,                                                                       
 19     Head(T
    }

    fn push_back(&mut self, value: T) {
        // add an element with value T to the back of the skiplist.
        todo! {}
    }

    fn random_level() -> usize {
        let mut rng = thread_rng();
        let mut level = 0;
        while rng.gen_bool(0.5) {
            level += 1;
        }
        level
    }
}
// https://www.epaperpress.com/sortsearch/download/skiplist.pdf
