use rug::{rand::RandState, Assign, Integer, integer::IsPrime};

fn function(n: Integer) -> Integer {

    let mut rand = RandState::new();
    
    loop {
        let mut candidate = Integer::from(Integer::random_bits(1024, &mut rand));
        candidate.set_bit(0, true);
        if IsPrime(candidate) {
            return candidate;
        }
    }
}
