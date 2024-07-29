/// insertion sort.
///
///  # Arguments:
///
/// `array: &mut Vec<T>` an unsorted array, where
/// T must implement `Ord` and `Copy`
pub fn insertion_sort<T: Ord + Copy>(array: &mut Vec<T>) {
    for i in 1..array.len() {
        let current = array[i];
        let mut j = (i as isize) - 1;

        // Move elements of array[0...i-1], that are greater than key,
        // to one position ahead of their current position
        while j >= 0 && array[j as usize] > current {
            array[(j + 1) as usize] = array[j as usize];
            j -= 1;
        }

        // Place the current element at its correct position
        array[(j + 1) as usize] = current;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }

    #[test]
    fn test_insertion_sort_with_string() {
        let mut arr = vec!["a", "c", "d", "g", "e"];
        insertion_sort(&mut arr);
        assert_eq!(vec!["a", "c", "d", "e", "g"], arr);
    }
}
