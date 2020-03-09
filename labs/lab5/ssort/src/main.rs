mod optimized;
mod unoptimized;
fn main() {}

/* The unoptimized selection sort took about 53.494 ms
 * while the optimized one took about 20.203 ms
 *
 * A zero cost abstraction means that there is no penalty for writing
 * something in a way that is at a higher level/abstraction.
 *
 * This is what we did when writing selection sort in a more functional manner
 * We used high level functions such as min_by_key, and map
 * and compared to writing efficient low-level code, we did not get
 * experience a performance penalty for using these
 */
