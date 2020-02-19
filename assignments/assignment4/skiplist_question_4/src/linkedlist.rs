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

#[derive(Debug, PartialEq)]
pub enum LinkedList<T> {
    Tail,
    Head(T, Box<LinkedList<T>>),
}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Self::Tail
    }

    pub fn new(t: T) -> Self {
        Self::Tail.push(t)
    }

    pub fn push(self, t: T) -> Self {
        Self::Head(t, Box::new(self))
    }

    pub fn push_back(self, t: T) -> Self {
        match self {
            Self::Head(curr, next) => next.push_back(t).push(curr),
            Self::Tail => Self::new(t),
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
        match self {
            Self::Head(curr, next) => {
                write!(f, "{}", curr).unwrap();
                match **next {
                    Self::Head(_, _) => write!(f, "=>{}", next),
                    Self::Tail => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn macro_test() {
        assert_eq!(
            LinkedList::Head(
                1,
                Box::new(LinkedList::Head(
                    2,
                    Box::new(LinkedList::Head(
                        3,
                        Box::new(LinkedList::Head(
                            5,
                            Box::new(LinkedList::Head(7, Box::new(LinkedList::Tail)))
                        ))
                    ))
                ))
            ),
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
}
