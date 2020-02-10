/* Original
#[derive(Debug, PartialEq)]
struct TreeNode {
    data: &str,
    left_child: Option<TreeNode>,
    right_child: Option<TreeNode>,
}
*/

// Fixed
#[derive(Debug, PartialEq)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

/*
 * The struct holds a reference (&str) so it requires a lifetime annotation
 * This annotation means an instance of TreeNode can’t outlive the reference it holds in its part field.
 * The left child and right child are not also references, since the struct has a recursive
 * definition and there is no way to know the exact size for it to go on the stack, so to the heap
 * it goes
 * i.e.
 *  if a struct will contain borrowed values (val), we must tell the compiler how long they're expected to last. The borrow checker can then enforce that any use of the val field obeys the restriction. The second issue is simply that Rust cannot know how much memory to allocate to the data type if it recursively contains itself. The fix is simply to fill this field with a pointer instead of an instance of the type.
https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
 */

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        if self.data == data {
            return;
        }

        let new_node = if data < self.data {
            &mut self.left_child
        } else {
            &mut self.right_child
        };

        match new_node {
            &mut Some(ref mut child) => child.insert_node(data),
            &mut None => {
                let node = TreeNode {
                    data,
                    left_child: None,
                    right_child: None,
                };
                let boxed = Some(Box::new(node));
                *new_node = boxed;
            }
        }
    }
}

#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}

/* Empty makes it so that instead of using an option, the tree node can either
 * be a node or an Empty type. The nice thing about this one is that it is immutable
 * and the data can be of any type that implements the Ord trait.
 */
impl<T: Ord> Tree<T> {
    fn insert_node(&mut self, new_data: T) {
        match self {
            &mut Tree::Node {
                ref data,
                ref mut left_child,
                ref mut right_child,
            } => match new_data.cmp(data) {
                std::cmp::Ordering::Less => right_child.insert_node(new_data),
                std::cmp::Ordering::Greater => left_child.insert_node(new_data),
                _ => return,
            },
            &mut Tree::Empty => {
                *self = Tree::Node {
                    data: new_data,
                    left_child: Box::new(Tree::Empty),
                    right_child: Box::new(Tree::Empty),
                }
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_treenode() {
        let mut x = TreeNode {
            data: "m",
            left_child: None,
            right_child: None,
        };
        x.insert_node("z");
        x.insert_node("b");
        x.insert_node("c");
        assert!(
            x == TreeNode {
                data: "m",
                left_child: Some(Box::new(TreeNode {
                    data: "b",
                    left_child: None,
                    right_child: Some(Box::new(TreeNode {
                        data: "c",
                        left_child: None,
                        right_child: None
                    })),
                })),
                right_child: Some(Box::new(TreeNode {
                    data: "z",
                    left_child: None,
                    right_child: None
                })),
            }
        );
    }
}
