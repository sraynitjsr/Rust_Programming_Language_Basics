use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct PriorityItem {
    priority: i32,
    value: String,
}

impl Ord for PriorityItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PriorityItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(Reverse(PriorityItem { priority: 5, value: "five".to_string() }));
    priority_queue.push(Reverse(PriorityItem { priority: 3, value: "three".to_string() }));
    priority_queue.push(Reverse(PriorityItem { priority: 8, value: "eight".to_string() }));
    priority_queue.push(Reverse(PriorityItem { priority: 2, value: "two".to_string() }));
    priority_queue.push(Reverse(PriorityItem { priority: 10, value: "ten".to_string() }));

    while let Some(Reverse(item)) = priority_queue.pop() {
        println!("{:?}", item);
    }
}
