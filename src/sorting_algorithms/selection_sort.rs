/// selection sort algorithm:
/// 1. find the smallest element in the array.
/// 2. swap it with the first element
/// 3. repeat for other elements
///
/// # Arguments:
/// `array: &mut Vec<T>`, and unsorted array, where T must
/// implement trait `PartialOrd`
pub fn selection_sort<T: PartialOrd>(array: &mut Vec<T>) {
    for i in 0..array.len() - 1_usize {
        // find the smallest in the rest of the array
        let mut smallest = i;
        for j in i..array.len() {
            if array[j] < array[smallest] {
                smallest = j;
            }
        }
        array.swap(i, smallest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }
}
