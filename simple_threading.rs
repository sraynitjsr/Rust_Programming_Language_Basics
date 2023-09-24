use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from the new thread!");
    });

    handle.join().unwrap();
    
    println!("Main thread continues.");
}
