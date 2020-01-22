use rug::Integer;

mod part3;
mod part4;


fn main() {

    println!("\nCalling the function for Q2 a couple times");
    println!("{}", part3::function(Integer::from(10)));                         
    println!("{}", part3::function(Integer::from(20)));                         
    println!("{}", part3::function(Integer::from(30)));                         
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));                        
    println!("{}", part3::function(Integer::from(100)));

    println!("\n\n\n\n\n\n");

    println!("Answer for part4");
    part4::prime_fun();
}
