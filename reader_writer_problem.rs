use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

struct SharedData {
    data: Vec<i32>,
    readers: usize,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            data: vec![1, 2, 3, 4, 5],
            readers: 0,
        }
    }

    fn read(&mut self) -> i32 {
        self.readers += 1;
        self.data[0]
    }

    fn write(&mut self, new_data: Vec<i32>) {
        self.data = new_data;
    }
}

fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData::new()));

    let (writer_tx, writer_rx) = mpsc::channel();
    let (reader_tx, reader_rx) = mpsc::channel();

    let shared_data_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        let new_data = vec![6, 7, 8, 9, 10];
        let mut shared_data = shared_data_clone.lock().unwrap();
        shared_data.write(new_data);
        writer_tx.send(()).unwrap();
    });

    for _ in 0..3 {
        let reader_tx = reader_tx.clone();
        let shared_data_clone = Arc::clone(&shared_data);

        thread::spawn(move || {
            let mut shared_data = shared_data_clone.lock().unwrap();
            let data = shared_data.read();
            reader_tx.send(data).unwrap();
        });
    }

    for _ in 0..3 {
        reader_rx.recv().unwrap();
    }
    writer_rx.recv().unwrap();

    let shared_data = shared_data.lock().unwrap();
    println!("Final Data => {:?}", shared_data.data);
}
