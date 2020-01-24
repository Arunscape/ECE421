fn functionalfun (x : isize) -> isize {
    (1..=x).map(|i| i*i + 2).sum()
}
