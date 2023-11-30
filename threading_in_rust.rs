use std::thread;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 1: Count {}", i);
            thread::sleep_ms(1000);
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 2: Count {}", i);
            thread::sleep_ms(800);
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread exiting.");
}
use std::thread;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 1: Count {}", i);
            thread::sleep_ms(1000);
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 2: Count {}", i);
            thread::sleep_ms(800);
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main thread exiting.");
}
