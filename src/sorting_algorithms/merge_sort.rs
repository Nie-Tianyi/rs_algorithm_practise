/// merge sort, a classic recursive sorting algorithm.
/// divide the original array into two pieces and sort them respectively.
/// it consumes the original array and returns a sorted array
///
/// # Arguments:
/// `mut array: Vec<T>` where T must implement trait `Ord`
///
/// # Returns:
/// `Vec<T>` this function returns a sorted array
pub fn merge_sort<T>(mut array: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    if array.len() <= 1 {
        return array;
    }

    let mut res: Vec<T> = Vec::with_capacity(array.len());

    // chop the original array into two pieces of equal size
    let b = array.split_off(array.len() / 2);

    let a = merge_sort(array); // recursively merge sort the left part
    let b = merge_sort(b); // recursively merge sort the right part

    // into iterators
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    // get the first element of each iterator
    let mut a_peak = a_iter.next();
    let mut b_peak = b_iter.next();

    loop {
        match (a_peak.take(), b_peak.take()) {
            (Some(a_val), Some(b_val)) => match a_val.cmp(&b_val) {
                std::cmp::Ordering::Less => {
                    res.push(a_val);
                    a_peak = a_iter.next();
                    b_peak = Some(b_val);
                }
                std::cmp::Ordering::Greater => {
                    res.push(b_val);
                    b_peak = b_iter.next();
                    a_peak = Some(a_val);
                }
                std::cmp::Ordering::Equal => {
                    res.push(a_val);
                    res.push(b_val);
                    a_peak = a_iter.next();
                    b_peak = b_iter.next();
                }
            },
            (None, Some(b_val)) => {
                res.push(b_val);
                res.extend(b_iter);
                return res;
            }
            (Some(a_val), None) => {
                res.push(a_val);
                res.extend(a_iter);
                return res;
            }
            (None, None) => {
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let arr = vec![5, 4, 3, 2, 1];
        let arr = merge_sort(arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }
}
