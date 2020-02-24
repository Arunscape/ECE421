use std::{cmp, fmt, mem};

use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct SkipList<T> {
    head: Box<SkipNode<T>>,
    len: usize,
    level_generator: LevelGenerator,
}

#[derive(Clone, Debug)]
pub struct SkipNode<D> {
    pub data: Option<D>, // only head can be none
    pub level: usize,
    pub next: Option<Box<SkipNode<D>>>,
    pub back: Option<*mut SkipNode<D>>,
    pub links: Vec<Option<*mut SkipNode<D>>>, // link to next node at level
    pub length_links: Vec<usize>,
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
    pub fn give_head() -> Self {
        SkipNode {
            data: None,
            level: 0,
            next: None,
            back: None,
            links: std::iter::repeat(None).take(0).collect(),
            length_links: std::iter::repeat(0).take(0).collect(),
        }
    }

    // TODO initialize next and back
    pub fn new(data: D, level: usize) -> Self {
        SkipNode {
            data: Some(data),
            level,
            next: None,
            back: None,
            links: std::iter::repeat(None).take(level + 1).collect(),
            length_links: std::iter::repeat(0).take(level + 1).collect(),
        }
    }

    /*
    pub fn into_inner(self) -> Option<V> {
        if self.value.is_some() {
            Some(self.value.unwrap())
        }
        None
    }
    */

    pub fn is_head(&self) -> bool {
        self.back.is_none()
    }
}

impl<D> fmt::Display for SkipNode<D>
where
    D: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        if let Some(ref v) = self.data {
            write!(f, "{}", v)
        } else {
            Ok(())
        }
    }
}

impl<T> SkipList<T> {
    pub fn new() -> Self {
        SkipList {
            head: Box::new(SkipNode::give_head()),
            len: 0,
            level_generator: LevelGenerator::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        // add an element with value T (strat from the beginning of the skiplist).
        unsafe {
            self.len += 1;
            let level = self.level_generator.random_level();
            let mut new_node = Box::new(SkipNode::new(value, level));
            let new_node_ptr: *mut SkipNode<T> = mem::transmute_copy(&new_node);
            // At each level, `node` moves down the list until it is just prior
            // to where the node will be inserted.  As this is parsed top-down,
            // the link lengths can't yet be adjusted and the insert nodes are
            // stored in `insert_nodes`.
            let mut node: *mut SkipNode<T> = mem::transmute_copy(&self.head);
            let mut insert_nodes: Vec<*mut SkipNode<T>> = Vec::with_capacity(new_node.level);

            let mut index_sum = 0;
            let mut level = level;
            while level > 0 {
                level -= 1;

                // Move insert_node down until `next` is not less than the new
                // node.
                while let Some(next) = (*node).links[level] {
                    if index_sum + (*node).length_links[level] < 0 {
                        // insert at front
                        index_sum += (*node).length_links[level];
                        node = next;
                        continue;
                    } else {
                        break;
                    }
                }
                // The node level is really just how many links it has. If we've
                // reached the node level, insert it in the links:
                // ```
                // Before:    [0] ------------> [1]
                // After:     [0] --> [new] --> [1]
                // ```
                if level <= new_node.level {
                    insert_nodes.push(node);
                    new_node.links[level] = (*node).links[level];
                    (*node).links[level] = Some(new_node_ptr);
                } else {
                    (*node).length_links[level] += 1;
                }
            }
        }
    }
}
