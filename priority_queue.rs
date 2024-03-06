use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};
use std::fmt;

struct PriorityItem {
    priority: i32,
    value: String,
}

impl fmt::Debug for PriorityItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PriorityItem")
            .field("priority", &self.priority)
            .field("value", &self.value)
            .finish()
    }
}

impl PartialOrd for PriorityItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Ord for PriorityItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialEq for PriorityItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityItem {}

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
