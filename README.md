# Algorithm and Data Structure Rust Ver. Notes

## Sorting Algorithms

### 1. Bubble Sort

Bubble sort is one of the simplest sorting algorithms, the time complexity of this sorting algorithms is $O(n^2)$

the pseudocode of this sorting algorithm can be expressed as:

```pseudocode
for i in range from 0 to array_length:
	for j in range from 0 to array_length - i -1:
		swap array[j] with array[j+1] if array[j] > array[j+1]
```

A better version of this algorithm is added a flag in the first iteration. If the array is already sorted, then return it directly.

### 2. Selection Sort

Selection Sort is another simple sorting algorithm. it selects the smallest element in the rest of the array and place it ahead. its pseudocode can be expresses as:

```pseudocode
for i in range from 0 to array_length -1:
	find index of the smallest element in the rest of the array,
	swap(i,smallest element)
```

The time complexity is also $O(n^2)$, because finding the smallest element in an array needs to iterate over the array.

### 3. Insertion Sort

Insertion Sort is a simple sorting algorithm as well. It picks one element at a time and inserts the element into the place where it should be. The pseudocode can be written as:

```pseudocode
for i in the range from 1 to array_length:
	let current element be the element at index i
	while array[i-1] < array[i], swap (i-1,i), i -= 1.
```

The time complexity of this algorithm is $O(n^2)$.

### 4. Merge Sort

Merge Sort is a recursive sorting algorithm. It divides the original array into two pieces of equal size, and sort them respectively. The pseudocode of this algorithm is:

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

The time complexity of this sorting algorithm is $O(n\log_2 n)$ , but it takes larger memory space than the first three sorting algorithms. the space complexity of this algorithm is $O(n)$ , while the space complexity of the first three algorithm is $O(1)$

### 5. Quick Sort

Quick Sort is another recursive sorting algorithm. It uses a unique techniques called "pivot". "pivot" is select an element in an array, and moves it to its rights place, where all the elements before it are smaller than it, and all the elements after it are greater than it. The pseudocode for **pivot** can be expressed as:

```pseudocode
select an element in the array, lets call it pivot point (usually the first element)
for every elements in array:
	if the element is smaller than pivot point:
		moves it before the pivot point,
```

The pseudocode for the **Quick Sort** then can be expressed as:

```pseudocode
pivot the array.
recursively pivot the elements on the right side of the pivot point,
recursively pivot the elements on the left side of the pivot point.
```

In the average cases, the time complexity of this algorithm is $O(n\log_2 n)$ and the space complexity of this algorithm is $O(n)$

## Data Structure

### 1. Stack

A stack is a linear data structure that follows the **Last In, First Out (LIFO)** principle. This means that the last element added to the stack will be the first one to be removed. 

![stack-operations](assets/images/stack-operations.png)

### 2. Linked List

![Linked List](assets/images/linked-list.webp)

A **linked list** is a linear data structure that consists of a series of nodes connected by pointers. Each node contains **data** and a **reference** to the next node in the list. Unlike arrays, linked lists allow for **efficient insertion or removal** of elements from any position in the list, as the nodes are not stored contiguously in memory.

There three types of linked list: singly linked list, doubly linked list, and circular linked list. singly linked list is the simplest one, it only contains a reference to the next node. doubly linked list contains two references, one to the next node, and one to the previous node. circular linked list is a linked list where the last node points to the first node. The time complexity for adding or deleting a node in a linked list is **$O(1)$** , the time complexity for insertion is $O(n)$.

In the rust implementation, to avoid the recursive reference, we use the `Rc<T>` to wrap the reference to the next node, and `Weak<T>` to wrap the reference to the previous node.

### 3. Queue

![queue](assets/images/queue.png)

A **Queue** is a fundamental data structure in computer science that follows the **First In, First Out (FIFO)** principle. This means that the first element added to the queue will be the first one to be removed. Queues are used in various applications such as scheduling processes in operating systems, managing tasks in a print spooler, and handling asynchronous data transfers.

### 4. Binary Tree

![binary tree](assets/images/binary-tree.png)

A **Binary Tree**, also been called Binary Search Tree, is a hierarchical data structure in which each node has at most two children, referred to as the left child and the right child. This structure is particularly useful for representing hierarchical relationships and is fundamental in various algorithms and applications. The time complexity of search a given element in a binary tree is $$O(\log_2n)$$ if the tree is balanced. The term "balanced" refers to a tree structure where the height difference between the left and right subtrees of any node is minimized. In the worst case, consider insert node 1,2,3,4,... in sequence, the binary will degrade to a singly linked list, where the time complexity of searching degrades to $$O(n)$$.

#### 4.1 Heap - A Complete Binary Tree

What is a Heap ? Heap is complete binary tree. A complete binary tree is a special type of binary tree where all the levels of the tree are filled completely except the lowest level nodes which are filled from as left as possible. How we represent a Heap? we use a `Vec`represent a Heap. for each node, it parent node's index in the `Vec` is $\lfloor\frac{n-1}{2}\rfloor$,its left child node's index is $2n+1$,right child node index is $2n+2$.

Heap can be classified into 2 class, Min Heap and Max Heap. In the Max Heap, every parent node's value is larger than its child nodes' value. In the Min Heap, every parent node's value is smaller than its child node's value. Min Heap is also called priority queue.

#### 4.2 AVL Tree - A Self-Balancing Binary Search Tree

An AVL Tree is a self-balancing tree, it will balance itself at every insertion. After each insertion, it will check the height difference between the left and right subtree of each node, if the difference is larger than 1, it will rotate the tree to make it balanced. The time complexity of searching in an AVL tree is constantly at $$O(\log_2n)$$.

![rotation](assets/images/rotation.png)

**_Pseudocode for Rotations_**

- Right Rotation

```pseudocode
right_rotate(node):
    left_child = node.left
    node.left = left_child.right
    left_child.right = node
    update_height(node)
    update_height(left_child)
    return left_child
```

- Left Rotation

```pseudocode
left_rotate(node):
    right_child = node.right
    node.right = right_child.left
    right_child.left = node
    update_height(node)
    update_height(right_child)
    return right_child
```

### 5. HashMap

A HashMap is a data structure that stores key-value pairs. It allows for fast retrieval, insertion, and deletion of values based on their keys. The keys are hashed to generate an index where the value is stored, making these operations generally very efficient. The time complexity for searching operation in a hash-map only takes $O(1)$ .

The core idea of HashMap is classify data into different buckets according to a hash algorithm, and when searching the data, search the data in its bucket directly. The bucket here is implemented by a `Vec` , and the `Bucketlist` is a `Vec` of `Vec`.  



## Supplementary Materials

### 1. A Stepper

we can use for-loop iterate over our struct if we have implemented trait `Iterator` for our struct:

```rust
pub struct Stepper{
    current: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max{
            return None;
        }
        let res = self.current;
        self.current += self.step;
        Some(res)
    }
}
```

### 2. Pseudo Random Generator Algorithms

Generating a pseudo random number involves take modulo on large integer. To generate a large integer, we can define few steps to calculate a random integer:

```rust
pub struct RandomGen {
    cur: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandomGen {
    pub fn new(seed: usize) -> Self {
        RandomGen {
            cur: seed,
            mul: 56394237_usize,
            inc: 346423496_usize,
            modulo: 25254463563_usize,
        }
    }

    pub fn rand_usize(&mut self, max: usize) -> usize {
        self.cur = (self.cur * self.mul + self.inc) % self.modulo;
        self.cur % max
    }
}
```

Here, the `mul`,`inc`,`modulo` are just random numbers. In case of overflow, we can use external crate to define these integers to be large numbers (e.g. array of i32 that implements basic math operations).

### 3. Hash Algorithm

Building a hash method for a struct in Rust usually involves using trait `Hash` and `Hasher` in the standard library. The `Hasher` defines the steps to compute a hash value, **any struct that implements `Hasher` trait can be used as a hash algorithm**. We normally do not need to implement this trait ourselves, instead we would use `std::collections::hash_map::DefaultHasher` directly. Here is an example of implementing a `Hasher` trait:

```rust
pub struct MHash {
    prev: u8,
    n: u128,
}

impl Hasher for MHash {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.n = (self.n + 11) * (*byte as u128 + 13)
                + ((byte ^ self.prev) as u128) % (u64::MAX as u128);
            self.prev = *byte;
        }
    }

    fn finish(&self) -> u64 {
        self.n as u64
    }
}
```

The `fn write(&mut self, bytes: &[u8])` method defines steps to update an internal value, and `fn finish(&self) -> u64` return the ultimate result.

The default hash method and hasher return a `u64` value. This is because that the build-in hash traits are normally target at finding buckets for data structures like HashMap and not for any cryptography usage. It is a trade off between efficiency and security. If you need higher securities, consider using a third party cryptographic crate which may provide common cryptographic hash algorithms like SHA256 or SHA512.

`Hash` trait defines an method that convert a type to a hash value. **Any struct that implements this trait can be used as the key of a HashMap / HashSet**. Here is definition of `Hash` trait:

```rust
pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);
}
```
