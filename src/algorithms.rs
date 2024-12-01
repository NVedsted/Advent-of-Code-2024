/// Counts the amount of times `x` is present in the slice
/// If the slice is not sorted, the returned result is unspecified and meaningless.
pub fn binary_count<T: Ord>(s: &[T], x: &T) -> usize {
    let low = s.partition_point(|v| v < x);
    let high = s.partition_point(|v| v <= x);

    high - low
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_count() {
        let haystack = [2, 2, 3, 3, 4, 5, 5, 5, 5, 6, 7, 8, 9, 9, 9];

        assert_eq!(super::binary_count(&haystack, &1), 0);
        assert_eq!(super::binary_count(&haystack, &2), 2);
        assert_eq!(super::binary_count(&haystack, &3), 2);
        assert_eq!(super::binary_count(&haystack, &4), 1);
        assert_eq!(super::binary_count(&haystack, &5), 4);
        assert_eq!(super::binary_count(&haystack, &6), 1);
        assert_eq!(super::binary_count(&haystack, &7), 1);
        assert_eq!(super::binary_count(&haystack, &8), 1);
        assert_eq!(super::binary_count(&haystack, &9), 3);
        assert_eq!(super::binary_count(&haystack, &10), 0);
    }
}
