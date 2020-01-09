fn functionalfun (x : isize) -> isize {
    return (1..=x).map(|i| i*i + 2).sum();
}
