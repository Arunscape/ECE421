pub fn selection_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        let min_idx = array[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(i, _)| i)
            .unwrap();

        array.swap(i, min_idx + i);
    }
}

#[cfg(test)]
#[test]
pub fn ssort_test() {
    let mut list = vec![6, 3, 7, 1, 4, 2, 0, 5, 9, 8];
    selection_sort(&mut list);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], list,)
}
