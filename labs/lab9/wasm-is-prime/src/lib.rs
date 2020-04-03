mod utils;
use primes;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-is-prime!");
}

#[wasm_bindgen]
pub fn CheckPrime(s: &JsValue) {
    let mut input: String = s.as_string().unwrap();
    if (is_prime(input)) {
        alert("Input is Prime");
    } else {
        alert("Input is Not Prime");
    }
}
pub fn is_prime(s: String) -> bool {
    primes::is_prime(s.parse().expect("non-numeric input entered"))
}
