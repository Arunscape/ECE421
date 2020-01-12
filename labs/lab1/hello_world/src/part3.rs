use rug::{rand::RandState, Assign, Integer, integer::IsPrime};

pub fn function() -> Integer {

    let mut rand = RandState::new();
    
    loop {
        let mut candidate = Integer::from(Integer::random_bits(1024, &mut rand));
        candidate.set_bit(0, true);
        if candidate.is_probably_prime(15) == IsPrime::Yes {
            return candidate;
        }
    }
}
