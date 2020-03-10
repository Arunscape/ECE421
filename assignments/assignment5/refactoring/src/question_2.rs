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

/// The function foo takes in two string slices.
/// The first string slice is meant to be multiple lines of text, while the
/// second string slice is meant to be a substring that may or may not appear
/// in one or more lines of the first string slice. In the example given, the
/// first string slice is an exerpt from Shakespeare, while the second string
/// slice is the word "the".
///
/// For each line in the first string slice, if the second string slice is found
/// within the first string slice, the function foo will print the line number
/// in which the second string slice was found, and also the position in that
/// line in which the second string slice was found.
///
/// For example, in the first line "Shall I compare thee to a summer's day?",
/// The line number is 0, and the substring "the" is found at position 16
/// within this line. So, x: 0, y: 16 will be printed out.
///
/// If no substring match is found on a given line, nothing is printed out.
///
/// The full output for the example is:
/// ```
/// x : 0, y : 16
/// x : 2, y : 21
/// x : 4, y : 18
/// x : 12, y : 23
/// x : 13, y : 42
/// ```
pub fn foo(text: &str, string: &str) -> Vec<L> {
    text.lines()
        .enumerate()
        .flat_map(|(line_number, line)| {
            line.match_indices(string)
                .map(move |(match_position, _)| (line_number, match_position))
        })
        .map(|(x, y)| L { x, y })
        .collect()
}
//TODO write 3 versions of this for question 2

pub fn main() {
    let results = foo(TEXT, "the");
    for x in results {
        println!("x : {}, y : {}", x.x, x.y);
    }
}

#[cfg(test)]
#[test]
fn q2_test() {
    let expected = vec![
        L { x: 0, y: 16 },
        L { x: 2, y: 21 },
        L { x: 4, y: 18 },
        L { x: 12, y: 23 },
        L { x: 13, y: 42 },
    ];

    assert_eq!(expected, foo(TEXT, "the"));
}
