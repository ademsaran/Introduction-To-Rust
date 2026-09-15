use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    {
        let (tx, rx) = mpsc::channel();

        // Creating a new thread and moving the transmitter (tx) into it
        // The move keyword is used to transfer ownership of tx to the new thread
        // This allows the new thread to send messages through the channel
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // Receiving the message sent from the spawned thread
        // The main thread will block here until it receives a message from the channel
        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
    {
        // Sending multiple values from one thread to another using channels
    
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
    {
        // Using multiple producers to send messages to a single receiver
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
    {
        // Shared state concurrency using Mutex
        // Ten threads will increment the shared counter by 1, and the final result should be 10
        // Arc (Atomic Reference Counting) is used to share ownership of the counter across multiple threads
        
        use std::sync::{Arc, Mutex};
        use std::thread;
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
