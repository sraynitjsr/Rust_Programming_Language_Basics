use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let sender_clone = sender.clone();

    thread::spawn(move || {
        sender_clone.send("Hello from the child thread").unwrap();
    });

    let message = receiver.recv().unwrap();
    println!("Received: {}", message);
}
