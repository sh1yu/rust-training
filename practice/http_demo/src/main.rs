use log::info;
use std::net::TcpListener;
use http_demo::handle_connection;
use http_demo::ThreadPool;

fn main() {
    simple_logger::init().unwrap();

    info!("Starting server...");

    let ip = "127.0.0.1:8080";
    let listener = TcpListener::bind(ip).expect("Unable to create listener.");
    info!("Server started on: {}{}", "http://", ip);

    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {  
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    } 
    info!("Shutting down.");
}
