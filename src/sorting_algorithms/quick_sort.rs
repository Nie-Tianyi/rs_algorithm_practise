use rayon;
/// pivot function takes the first element in an array,
/// and moves it to its right place.
/// every element smaller than the first element
/// should be before it, and every element
/// greater than it, should be after it.
///
/// # Arguments:
/// `array:&mut [T]`: array to be sorted
///
/// # Returns:
/// `usize`: the index of the first elements after being moved
pub fn pivot<T: Ord>(array: &mut [T]) -> usize {
    // index of the first element
    let mut pivot = 0_usize;

    // iterate over the rest of the elements
    for i in 1..array.len() {
        // if the element is smaller than pivot element
        if array[i] < array[pivot] {
            // swap it with the next element of pivot element
            array.swap(i, pivot + 1);
            // swap pivot with its next element
            array.swap(pivot, pivot + 1);
            // make pointer plus one
            pivot += 1;
        }
    }

    pivot
}

/// quick sort is another recursive sorting algorithm.
/// it picks up the first element of the array, and
/// moves it to its right place. then sort the rest of
/// the array with the same method.
///
/// # Arguments:
///
/// `array: &mut [T]`: array to be sorted, a mutable slice. T must
/// implements trait `Ord`
pub fn quick_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1_usize {
        return;
    }

    let pivot_point = pivot(array);
    let (left, right) = array.split_at_mut(pivot_point);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

/// rayon version of quick sort, speed up by thread pool.
/// # Arguments:
///
/// `array: &mut [T]`: array to be sorted, a mutable slice. T must
/// implements trait `Ord` and `Send`.
/// 
/// `Send` is an auto trait, which means it will be implemented
/// automatically if all of its member variables has implemented
/// this trait
pub fn rayon_quick_sort<T: Ord + Send>(array: &mut [T]) {
    if array.len() <= 1_usize {
        return;
    }

    let pivot_point = pivot(array);
    let (left, right) = array.split_at_mut(pivot_point);

    rayon::join(
        || rayon_quick_sort(left),
        || rayon_quick_sort(&mut right[1..]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot() {
        let mut array = vec![3, 1, 4, 2, 5];
        let p = pivot(&mut array);
        assert_eq!(vec![1, 2, 3, 4, 5], array);
        assert_eq!(2_usize, p);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }

    #[test]
    fn test_rayon_quick_sort() {
        let mut arr = vec![5, 4, 3, 2, 1];
        rayon_quick_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }
}
