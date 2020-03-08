use std::{cmp, fmt, iter, mem};

use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct SkipList<T> {
    head: Box<SkipNode<T>>,
    last: Option<Box<SkipNode<T>>>,
    level: usize,
    length: usize,
    level_generator: LevelGenerator,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct SkipNode<D> {
    pub data: Option<D>,                          // only head can be none
    pub forwards: Vec<Option<*mut SkipNode<D>>>,  // link to next node at level
    pub backwards: Vec<Option<*mut SkipNode<D>>>, // link to back node at level
}

#[derive(Clone, Debug)]
struct LevelGenerator {
    rng: ThreadRng,
}

impl LevelGenerator {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

trait RandomLevel {
    fn random_level(&mut self) -> usize;
}

impl RandomLevel for LevelGenerator {
    fn random_level(&mut self) -> usize {
        let mut level = 1;
        while self.rng.gen_bool(0.5) {
            level += 1;
        }
        level
    }
}

impl<D> SkipNode<D> {
    pub fn new(data: D, level: usize) -> Self {
        SkipNode {
            data: Some(data),
            forwards: std::iter::repeat(None).take(level + 1).collect(),
            backwards: std::iter::repeat(None).take(level + 1).collect(),
        }
    }
    pub fn give_head(level: usize) -> Self {
        SkipNode {
            data: None,
            forwards: std::iter::repeat(None).take(level + 1).collect(),
            backwards: std::iter::repeat(None).take(level + 1).collect(),
        }
    }

    pub fn is_head(&self) -> bool {
        self.backwards[0].is_none()
    }

    pub fn level(&self) -> usize {
        self.forwards.len() - 1
    }

    pub fn get_right(&self, level: usize) -> Option<*mut SkipNode<D>> {
        match self.forwards.get(level) {
            Some(&ptr) => ptr,
            None => None,
        }
    }
    pub fn get_left(&self, level: usize) -> Option<*mut SkipNode<D>> {
        match self.backwards.get(level) {
            Some(&ptr) => ptr,
            None => None,
        }
    }
    pub fn set_right(&mut self, n: Option<*mut SkipNode<D>>, level: usize) {
        if let Some(_) = self.forwards.get(level) {
            self.forwards[level] = n;
        } else {
            self.forwards.insert(level, n);
        }
    }
    pub fn set_left(&mut self, n: Option<*mut SkipNode<D>>, level: usize) {
        if let Some(_) = self.backwards.get(level) {
            self.backwards[level] = n;
        } else {
            self.backwards.insert(level, n);
        }
    }
}

impl<D> fmt::Display for SkipNode<D>
where
    D: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        if let Some(d) = &self.data {
            write!(f, "{}", d)
        } else {
            Ok(())
        }
    }
}

impl<T: cmp::Ord> SkipList<T> {
    pub fn new() -> Self {
        SkipList {
            head: Box::new(SkipNode::give_head(1)),
            last: None,
            length: 0,
            level: 0,
            level_generator: LevelGenerator::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        // add an element with value T (strat from the beginning of the skiplist).
        unsafe {
            self.length += 1;

            // first insertion
            if self.length == 0 {
                let new_node = Box::new(SkipNode::new(value, 0));
                let new_node_ptr: *mut SkipNode<T> = mem::transmute_copy(&new_node);
                (*self.head).forwards.push(Some(new_node_ptr));
                return;
            }

            let level = self.level_generator.random_level();
            if self.level < level {
                self.level = level
            }

            // make the head have an appropriate number of levels
            while (*self.head).forwards.len() - 1 < level {
                (*self.head).forwards.push(None);
            }

            let new_node = Box::new(SkipNode::new(value, level));
            let new_node_ptr: *mut SkipNode<T> = mem::transmute_copy(&new_node);
            self.update_list_pointers(new_node_ptr, level);
        }
    }

    pub fn push_back(&mut self, value: T) {
        // add an element with value T (strat from the end of the skiplist).
        unsafe {
            self.length += 1;

            // first insertion
            if self.length == 0 {
                let new_node = Box::new(SkipNode::new(value, 0));
                let new_node_ptr: *mut SkipNode<T> = mem::transmute_copy(&new_node);
                (*self.head).backwards.push(Some(new_node_ptr));
                return;
            }

            let level = self.level_generator.random_level();
            if self.level < level {
                self.level = level
            }

            // make the head have an appropriate number of levels
            while (*self.head).forwards.len() - 1 < level {
                (*self.head).forwards.push(None);
            }

            let new_node = Box::new(SkipNode::new(value, level));
            let new_node_ptr: *mut SkipNode<T> = mem::transmute_copy(&new_node);
            self.update_list_pointers(new_node_ptr, level);
        }
    }

    fn update_list_pointers(&self, n: *mut SkipNode<T>, level: usize) {
        unsafe {
            for i in 0..=level {
                let left = self.search_closest_node(Some((*n).data.as_ref().unwrap()), i);
                (*n).set_right((*left).get_right(i), i);
                (*left).set_right(Some(n), i);
            }
        }
    }

    fn search_closest_node(&self, value: Option<&T>, minlevel: usize) -> *mut SkipNode<T> {
        unsafe {
            let mut n: *mut SkipNode<T> = mem::transmute_copy(&self.head);

            for i in (minlevel..=self.level).rev() {
                loop {
                    if let Some(right) = (*n).get_right(i) {
                        match value.cmp(&(*n).data.as_ref()) {
                            std::cmp::Ordering::Greater => n = right,
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }
            }
            n
        }
    }

    fn search_closest_node_back(&self, value: Option<&T>, minlevel: usize) -> *mut SkipNode<T> {
        unsafe {
            let mut n: *mut SkipNode<T> = mem::transmute_copy(self.last.as_ref().unwrap());
            if self.last.is_none() {
                return n;
            }

            for i in (minlevel..=self.level).rev() {
                loop {
                    if let Some(left) = (*n).get_left(i) {
                        match value.cmp(&(*n).data.as_ref()) {
                            std::cmp::Ordering::Less => n = left,
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }
            }
            n
        }
    }
}

impl<T> fmt::Display for SkipList<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut n: *mut SkipNode<T> = mem::transmute_copy(&self.head);

            while let Some(_) = (*n).forwards[0] {
                write!(f, "{:?}", (*n).data);
                n = (*n).forwards[0].unwrap();
            }
            Ok(())
        }
    }
}
