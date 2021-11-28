// lib.rs
use log::info;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    // let request = String::from_utf8_lossy(&buffer[..]);

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            info!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}



struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move|| {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        info!("Worker {} got a job; executing.", id);

                        job();
                    },
                    Message::Terminate => {
                        info!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}