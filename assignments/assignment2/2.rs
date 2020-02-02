struct Bag<T> {
 items: [T; 3],
}

#[allow(non_snake_case)]
fn BagSize<T>(_bag: Bag<T>) -> usize {
    std::mem::size_of::<Bag<T>>()
}

fn main() {
    let b1 = Bag {items: [1u8, 2u8, 3u8], };
    let b2 = Bag {items: [1u32, 2u32, 3u32], };

    println!("size of First Bag = {} bytes", BagSize(b1));
    println!("size of Second Bag = {} bytes", BagSize(b2));
}


