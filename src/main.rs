use std::io;
use std::net::UdpSocket;
use std::thread;
use std::time;

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:7878").unwrap();
    socket.set_nonblocking(true).unwrap();

    let mut buf = [0; 10];
    let (num_bytes_read, _) = loop {
        match socket.recv_from(&mut buf) {
            Ok(n) => break n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();

                println!("sleep ...");
                thread::sleep(time::Duration::from_secs(1));

                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    };
    println!("bytes: {:?}", &buf[..num_bytes_read]);
}
