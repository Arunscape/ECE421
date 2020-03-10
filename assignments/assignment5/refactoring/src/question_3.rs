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
        if let Some(_) = current_node.value {
            self.len -= 1; // already exists
        }
        current_node.value = Some(value);
    }

    fn length(&self) -> usize {
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = (String, i32)> {
        fn recurse_children(n: &TrieNode, s: &mut String, hashmap: &mut HashMap<String, i32>) {
            n.chs.iter().for_each(|(character, node)| match n.value {
                Some(v) => {
                    hashmap.insert(s.to_string(), v);
                    recurse_children(node, s, hashmap);
                }
                None => {
                    s.push(*character);
                    recurse_children(node, s, hashmap);
                }
            })
        }
        let mut hm: HashMap<String, i32> = HashMap::new();
        recurse_children(&self.root, &mut String::new(), &mut hm);

        hm.into_iter()
    }

    fn find(&self, key: &String) -> Option<&TrieNode> {
        fn recurse_children<'a>(n: &'a TrieNode, key: &str) -> Option<&'a TrieNode> {
            if key == "" {
                Some(n)
            } else {
                let (start, rest) = key.split_at(1);
                n.chs
                    .get(&start.chars().next().unwrap())
                    .and_then(|x| recurse_children(x, rest))
            }
        }
        recurse_children(&self.root, key)
    }

    fn delete(&mut self, key: &String) -> Option<i32> {
        fn recurse_children<'a>(n: &mut TrieNode, key: &str) -> Option<i32> {
            if key == "" {
                let ret = n.value;
                n.value = None;
                ret
            } else {
                let (start, rest) = key.split_at(1);
                n.chs
                    .get_mut(&start.chars().next().unwrap())
                    .and_then(|x| recurse_children(x, rest))
            }
        }
        let res = recurse_children(&mut self.root, key);
        if res.is_some() {
            self.len -= 1;
        }
        res
    }
}

pub fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);
    println!("{:#?}", trie);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn length() {
        let mut trie = Trie::new();
        trie.add_string("Foo".to_string(), 1);
        trie.add_string("Bar".to_string(), 2);
        trie.add_string("Baz".to_string(), 2);
        trie.add_string("Baz".to_string(), 2);

        assert_eq!(3, trie.length());
    }

    #[test]
    fn iter() {
        let expected: Vec<(String, i32)> = vec![
            ("one".into(), 1),
            ("two".into(), 2),
            ("three".into(), 3),
            ("four".into(), 4),
        ];
        let mut t = Trie::new();
        expected
            .iter()
            .for_each(|(s, v)| t.add_string(s.clone(), *v));

        assert!(t.iter().zip(expected.iter()).all(|(a, b)| a == *b));
    }

    #[test]
    fn find() {
        let expected: Vec<(String, i32)> = vec![
            ("one".into(), 1),
            ("two".into(), 2),
            ("three".into(), 3),
            ("four".into(), 4),
        ];
        let mut t = Trie::new();
        expected
            .iter()
            .for_each(|(s, v)| t.add_string(s.clone(), *v));

        assert_eq!(3, t.find("three").unwrap().value.unwrap());
        assert!(t.find("does not exist").is_none());
    }

    #[test]
    fn delete() {
        let expected: Vec<(String, i32)> = vec![
            ("one".into(), 1),
            ("two".into(), 2),
            ("three".into(), 3),
            ("four".into(), 4),
        ];
        let mut t = Trie::new();
        expected
            .iter()
            .for_each(|(s, v)| t.add_string(s.clone(), *v));

        assert_eq!(3, t.delete(&"three".to_owned()).unwrap());
        assert!(t.find("three").unwrap().value.is_none());
        assert_eq!(3, t.length());
    }
}
