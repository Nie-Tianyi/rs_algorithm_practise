# Algorithm and Data Structure Rust Ver. Notes

## Sorting Algorithms

### 1. Bubble Sort

Bubble sort is one of the simplest sorting algorithms, the time complexity of this sorting algorithms is $O(n^2)$

the pseudo code of the this sorting algorithm can be expressed as:

```pseudocode
for i in range from 0 to array_length:
	for j in range from 0 to array_length - i -1:
		swap array[j] with array[j+1] if array[j] > array[j+1] 
```

A better version of this algorithm is add a flag in the first iteration. If the array is already sorted, then return it directly.

### 2. Selection Sort

Selection Sort is another simple sorting algorithm. it selects the smallest element in the rest of the array and place it  ahead. its pseudo code can be expresses as:

```pseudocode
for i in range from 0 to array_length -1:
	find index of the smallest element in the rest of the array,
	swap(i,smallest element)
```

The time commplexity is also $O(n^2)$, because finding the smalles element in an array needs to iterate over the array.

### 3. Insertion Sort

Insertion Sort is a simple sorting algorithm as well. It picks one element at a time and inserts the element into the place where it should be.  The pseudo code can be written as:

```pseudocode
for i in the range from 1 to array_length:
	let current element be the element at index i
	while array[i-1] < array[i], swap (i-1,i), i -= 1.
```

The time complexity of this algorithm is $O(n^2)$.

### 4. Merge Sort

Merge Sort is a recursive sorting algorthm. It divides the orginal array into two pieces of equal size, and sort them respectively. The pseudo code of this algorithm is:

```pseudocode
divide the array into left half, right half:
if the length of the array is smaller or equal to 1, returns
merge sort the left half
marge sort the right half
init a new array to store the result
for elements in left half, and elements in right half
	Place the smaller elements of the two arrays one by one into the new array
returns the new array
	
```

The time complexity of this sorting algorithm is $O(nlog_2 n)$ , but it takes larger memory space than the first three sorting algorithms. the space complexity of this algorithm is $O(n)$ , while the space complexity of the first three algorithm is $O(1)$

