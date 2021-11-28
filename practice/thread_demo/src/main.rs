use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel();

    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || {
        println!("Here's a vector: {:?}", v);
        let val = String::from("Hi");
        tx.send(val).unwrap();
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    handle.join().unwrap();
}