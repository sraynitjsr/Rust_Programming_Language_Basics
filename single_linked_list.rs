use std::fmt;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

impl<T: fmt::Debug> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = &self.head;
        let mut result = String::new();
        while let Some(node) = current {
            result.push_str(&format!("{:?} -> ", node.data));
            current = &node.next;
        }
        result.push_str("None");
        write!(f, "{}", result)
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);

    println!("Linked list: {}", list);

    if let Some(value) = list.pop() {
        println!("Popped: {}", value);
    }

    println!("Linked list after pop: {}", list);
}
