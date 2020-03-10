#[derive(Debug, PartialEq)]
pub struct L {
    x: usize,
    y: usize,
}
const TEXT: &'static str = "Shall I compare thee to a summer's day?
Thou art more lovely and more temperate:
Rough winds do shake the darling buds of May,
And summer's lease hath all too short a date:
Sometimes too hot the eye of heaven shines,
And too often is his gold complexion dimm'd:
And every fair from fair sometimes declines,
By chance or natures changing course untrimm'd;
By thy eternal summer shall not fade,
Nor lose possession of that fair thou owest;
Nor shall Death brag thou wander'st in his shade,
When in eternal lines to time thou growest:
So long as men can breathe or eyes can see,
So long lives this and this gives life to thee.";

/// used iterators to get rid of the need to manually keep track of the line number
pub fn question_2a(text: &str, string: &str) -> Vec<L> {
    let mut r = Vec::new();
    for (line_number, line) in text.lines().enumerate() {
        for (match_position, _) in line.match_indices(string) {
            r.push(L {
                x: line_number,
                y: match_position,
            })
        }
    }
    r
}

/// using map to get rid of the need to manually update the r vector
pub fn question_2b(text: &str, string: &str) -> Vec<L> {
    let r: Vec<L> = text
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.match_indices(string)
                .map(move |(match_position, _)| (line_number, match_position))
        })
        .flatten()
        .map(|(x, y)| L { x, y })
        .collect();
    r
}

/// question 2c get rid of let statements
pub fn foo(text: &str, string: &str) -> Vec<L> {
    text.lines()
        .enumerate()
        .flat_map(|(x, line)| line.match_indices(string).map(move |(y, _)| L { x, y }))
        .collect()
}

pub fn main() {
    let results = foo(TEXT, "the");
    for x in results {
        println!("x : {}, y : {}", x.x, x.y);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn foo_test() {
        let expected = vec![
            L { x: 0, y: 16 },
            L { x: 2, y: 21 },
            L { x: 4, y: 18 },
            L { x: 12, y: 23 },
            L { x: 13, y: 42 },
        ];

        assert_eq!(expected, foo(TEXT, "the"));
    }

    #[test]
    fn q2a() {
        let expected = vec![
            L { x: 0, y: 16 },
            L { x: 2, y: 21 },
            L { x: 4, y: 18 },
            L { x: 12, y: 23 },
            L { x: 13, y: 42 },
        ];

        assert_eq!(expected, question_2a(TEXT, "the"));
    }

    #[test]
    fn q2b() {
        let expected = vec![
            L { x: 0, y: 16 },
            L { x: 2, y: 21 },
            L { x: 4, y: 18 },
            L { x: 12, y: 23 },
            L { x: 13, y: 42 },
        ];

        assert_eq!(expected, question_2b(TEXT, "the"));
    }
}
