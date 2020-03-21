use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

/// The reason the old code does not run is because sample_data gets moved into
/// a closure,
/// (i.e. that closure now owns it)
/// and then it is dropped one it goes out of scope of the closure.
/// However, we would like to have sample_data be used multiple times,
/// in many closures which are passed to different threads.
/// To fix this, I used an automatic reference counter, which keeps track
/// of the references to the shared object, as the name would suggest,
/// and a mutex to make sure only one thread can access the shared data at once.
fn main() {
    let mut sample_data = Arc::new(Mutex::new(vec![1, 81, 107]));
    for i in 0..10 {
        let sample_data = sample_data.clone();
        thread::spawn(move || {
            sample_data.lock().unwrap()[0] += i;
        }); // does not fail here anymore
    }
    thread::sleep(Duration::from_millis(50));
}

/* old code
fn main() {
    let mut sample_data = vec![1, 81, 107];
    for i in 0..10 {
        thread::spawn(move || {
            sample_data[0] += i;
        }); // fails here
    }
    thread::sleep(Duration::from_millis(50));
}
*/

/* compiler error:
error[E0382]: use of moved value: `sample_data`
  --> src/main.rs:23:23
   |
21 |     let mut sample_data = vec![1, 81, 107];
   |         --------------- move occurs because `sample_data` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
22 |     for i in 0..10 {
23 |         thread::spawn(move || {
   |                       ^^^^^^^ value moved into closure here, in previous iteration of loop
24 |             sample_data[0] += i;
   |             ----------- use occurs due to use in closure

error: aborting due to previous error
*/
