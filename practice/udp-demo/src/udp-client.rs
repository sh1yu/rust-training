use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:22222").expect("couldn't bind to address");
    let buf = [1u8; 60000];
    let mut count = 1;
    loop {
        socket.send_to(&buf[0..count], "127.0.0.1:8888").expect("couldn't send to remote address");
        count = count + 1;
        if count == 10 {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
