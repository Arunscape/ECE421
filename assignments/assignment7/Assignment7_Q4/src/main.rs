use rayon;

fn concurrent_quick_sort<T>(v: &mut [T])
where
    T: PartialOrd + Send,
{
    //add your code here:
    //uses partition fn, multiple options exist.
    //use rayon for concurrency.
    if v.len() < 2 {
        return;
    }
    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| concurrent_quick_sort(lo), || concurrent_quick_sort(hi));
}
fn partition<T>(v: &mut [T]) -> usize
where
    T: PartialOrd + Send,
{
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

fn main() {
    let mut v = vec![10, 80, 30, 90, 40, 50, 70];

    println!("Before: {:?}", v);

    concurrent_quick_sort(&mut v);

    println!("After: {:?}", v);
}
