#[allow(non_camel_case_types)]
struct Bag_u8 {
 items: [u8; 3],
}

#[allow(non_camel_case_types)]
struct Bag_u32 {
 items: [u32; 3],
}

#[allow(non_snake_case)]
fn BagSize_u8 (_: Bag_u8) -> usize{
    std::mem::size_of::<Bag_u8>()
}

#[allow(non_snake_case)]
fn BagSize_u32 (_: Bag_u32) -> usize{
    std::mem::size_of::<Bag_u32>()
}

fn main() {
    let b1 = Bag_u8 {items: [1u8, 2u8, 3u8], };
    let b2 = Bag_u32 {items: [1u32, 2u32, 3u32], };

    println!("size of First Bag = {} bytes", BagSize_u8(b1));
    println!("size of Second Bag = {} bytes", BagSize_u32(b2));
}



