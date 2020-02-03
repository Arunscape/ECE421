use std::mem::size_of_val;

fn main() {
    
    let vec1 = Box::new(vec![12, 32, 13]);
    let vec2 = Box::new(vec![44, 55, 16]);
    { 
        let vec1_iter = Box::new((*vec1).iter());
        println!("size of vec1_iter = {} bytes", size_of_val(&vec1_iter));
    }
    {
        let vec_chained = Box::new((*vec1).iter().chain((*vec2).iter()));
        println!("size of vec_chained = {} bytes", size_of_val(&vec_chained));
    }
    { 
        let vec1_2=Box::new(vec![*vec1, *vec2]);
        let vec_flattened = Box::new((*vec1_2).iter().flatten()); 
        println!("size of vec_flattened = {} bytes", size_of_val(&vec_flattened));
    }

}

/* Output: 
size of vec1_iter = 8 bytes
size of vec_chained = 8 bytes
size of vec_flattened = 8 bytes
*/
