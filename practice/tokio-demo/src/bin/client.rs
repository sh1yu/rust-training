// use tokio::net::{TcpStream};
// use std::net::SocketAddr;
// use tokio::io;

// pub type Error = Box<dyn std::error::Error + Send + Sync>;
// pub type Result<T> = std::result::Result<T, Error>;

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {

    // let addr = "127.0.0.1:6142".parse::<SocketAddr>().unwrap();
    // let stream = TcpStream::connect(&addr).await?;
    // Ok(())

    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
