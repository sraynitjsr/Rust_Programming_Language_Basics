use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut num_indices = HashMap::new();

    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = num_indices.get(&complement) {
            return Some((complement_index, index));
        }

        num_indices.insert(num, index);
    }

    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    match two_sum(nums, target) {
        Some((index1, index2)) => {
            println!("The indices of the two numbers that add up to {} are: {}, {}", target, index1, index2);
        }
        None => {
            println!("No solution found.");
        }
    }
}
