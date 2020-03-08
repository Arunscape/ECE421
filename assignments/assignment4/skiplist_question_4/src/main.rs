mod skiplist;

use crate::skiplist::SkipList;

fn main() {
    let mut list: SkipList<usize> = SkipList::new();

    println!("{:#?}", list);

    list.push(30);
    list.push(25);
    list.push(50);
}

/* README
 * I worked on question 4, but at this point, I need to study for midterms and
 * work on other assignments and labs. The program does not panic at run time
 * when the code above is run, but I am not sure if it is inserting items
 * correctly. I also tried to implement push_back, but I think it does not work
 * I'm hoping that what I've written so far is at least worth some part marks
 * 😊
 */
