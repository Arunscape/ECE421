use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
    len: usize,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
            len: 0,
        }
    }

    fn add_string(&mut self, string: String, value: i32) {
        self.len += 1;
        let mut current_node = &mut self.root;
        for c in string.chars() {
            current_node = current_node.chs.entry(c).or_insert(TrieNode {
                chs: HashMap::new(),
                value: None,
            });
        }
        current_node.value = Some(value);
    }

    fn length(&self) -> usize {
        self.len
    }
}

impl Iterator for Trie{
    fn next (&mut self) -> 
   match self.root.chs.len() {
    0 => None,
    _ => 
   } 
}

pub fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);
    println!("{:#?}", trie);
}

#[cfg(test)]
#[test]
fn q3_test() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);

    assert_eq!(2, trie.length());
}
