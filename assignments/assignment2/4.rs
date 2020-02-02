use std::mem::size_of_val;

fn main() {
    
    let vec1 = vec![12, 32, 13];
    let vec2 = vec![44, 55, 16];
    { 
        let vec1_iter = vec1.iter();
        println!("size of vec1_iter = {} bytes", size_of_val(&vec1_iter));
    }
    {
        let vec_chained = vec1.iter().chain(vec2.iter());
        println!("size of vec_chained = {} bytes", size_of_val(&vec_chained));
    }
    { 
        let vec1_2=vec![vec1, vec2];
        let vec_flattened = vec1_2.iter().flatten(); 
        println!("size of vec_flattened = {} bytes", size_of_val(&vec_flattened));
    }

}

/* Output: 
size of vec1_iter = 16 bytes
size of vec_chained = 40 bytes
size of vec_flattened = 64 bytes
*/
