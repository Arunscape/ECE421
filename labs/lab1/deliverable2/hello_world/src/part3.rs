use rug::{rand::RandState, Integer, integer::IsPrime};
use rand::Rng;

pub fn function(n: Integer) -> Integer {

    let mut rand = RandState::new();
    // setting the seed because otherwise the same "random" number is generated
    let mut idk = rand::thread_rng();
    let seed: u32 = idk.gen();
    rand.seed(&Integer::from(seed));
    
    loop {
        let num = n.clone();
        let mut candidate = num.random_below(&mut rand);

//        println!("RANDOM: {}", candidate);

        candidate.set_bit(0, true);
        if candidate.is_probably_prime(15) == IsPrime::Yes {
            return candidate;
        }
    }
}
