/// Bubble sort, most classic and simple sorting algorithm.
/// Sort array in ascending order.
/// time complexity of this algorithm is O(n^2)
///
/// # Arguments:
/// `array: &mut Vec<T>` is an unsorted array, where T
/// must implements trait `Ord`
pub fn bubble_sort<T: Ord>(array: &mut Vec<T>) {
    // if the only has one element, return directly
    if array.len() <= 1 {
        return;
    }

    for _i in 0..array.len() {
        for j in 0..array.len() - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

/// Bubble sort, but a better version,
/// including a flag, and need less iterations.
/// Sort array in ascending order.
///
/// # Arguments:
/// `array: &mut Vec<T>` is an unsorted array, where T
/// must implements trait `PartialOrd`
pub fn better_bubble_sort<T: PartialOrd>(array: &mut Vec<T>) {
    // if the only has one element, return directly
    if array.len() <= 1 {
        return;
    }

    let mut flag = true;
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                flag = false;
            }
        }
        if flag {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }

    #[test]
    fn test_better_bubble_sort() {
        let mut arr = vec![5, 2, 3, 4, 1];
        better_bubble_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }
}
