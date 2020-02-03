#[derive(Debug)]
struct Bag<T> {
 items: [T; 3],
}

fn main() {
 let b = Bag { items: [1, 2, 3] };
 let b2 = Bag { items: [vec![1, 2, 3], vec![2], vec![4, 5, 6]] };

 println!("{:?}", b);
 println!("{:?}", b2);
}
