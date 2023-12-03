use std::collections::{HashMap, BinaryHeap};
use std::collections::binary_heap::Reverse;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut frequency_map = HashMap::new();

    // Count the frequency of each element
    for &num in &nums {
        let counter = frequency_map.entry(num).or_insert(0);
        *counter += 1;
    }

    // Create a min-heap to store the k most frequent elements
    let mut min_heap = BinaryHeap::new();

    for (&num, &frequency) in &frequency_map {
        // Push elements into the min-heap
        min_heap.push(Reverse((frequency, num)));

        // If the size of the heap exceeds k, remove the smallest element
        if min_heap.len() > k as usize {
            min_heap.pop();
        }
    }

    // Extract the elements from the min-heap
    let mut result = Vec::new();
    while let Some(Reverse((_, num))) = min_heap.pop() {
        result.push(num);
    }

    result
}

fn main() {
    // Example usage
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = top_k_frequent(nums, k);
    println!("{:?}", result); // Output: [1, 2]
}
