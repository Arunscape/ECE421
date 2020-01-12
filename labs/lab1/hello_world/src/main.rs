use rand::prelude::*;

mod part3;

fn main() {
    let mut rng = thread_rng();
    if rng.gen() {
    let x: f64 = rng.gen();
    let y = rng.gen_range(-10.0, 10.0);
    println!("x is: {}", x);
    println!("y is: {}", y);
    println!("Random Number between 0 and 9: {}", rng.gen_range(0, 10));
    }

    println!("{}", part3::function());
    println!("{}", part3::function());
    println!("{}", part3::function());
}
