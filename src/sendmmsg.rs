
use std::mem;

use libc::*;

pub unsafe fn run() {
        let sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if sockfd == -1 {
        panic!("socket()");
    }

    let sa = sockaddr_in{
        sin_family: AF_INET as u16,
        sin_addr: in_addr { s_addr: htonl(INADDR_LOOPBACK) },
        sin_port: htons(1234),
        sin_zero: [0; 8],
    };

    if connect(sockfd, &sa as *const _ as *const sockaddr, mem::size_of_val(&sa) as socklen_t) == -1 {
        panic!("connect()");
    }


    let mut msg1: [iovec; 2] = [mem::zeroed(); 2];
    let mut one = "one".to_string();
    msg1[0].iov_base = one.as_mut_ptr() as *mut c_void;
    msg1[0].iov_len = 3;
    let mut two = "two".to_string();
    msg1[1].iov_base = two.as_mut_ptr() as *mut c_void;
    msg1[1].iov_len = 3;

    let mut msg2: iovec = mem::zeroed();
    let mut three = "three".to_string();
    msg2.iov_base = three.as_mut_ptr() as *mut c_void;
    msg2.iov_len = 5;

    let mut msg: [mmsghdr; 2] = [mem::zeroed(); 2];
    msg[0].msg_hdr.msg_iov = &mut msg1[0];
    msg[0].msg_hdr.msg_iovlen = 2;

    msg[1].msg_hdr.msg_iov = &mut msg2;
    msg[1].msg_hdr.msg_iovlen = 1;

    let retval = sendmmsg(sockfd, &mut msg[0] as *mut mmsghdr, 2, 0);
    if retval == -1 {
        panic!("sendmmsg()");
    } else {
        println!("> {}", retval);
    }
}

fn htonl(n: u32) -> u32 {
    n.to_be()
}

fn htons(n: u16) -> u16 {
    n.to_be()
}

