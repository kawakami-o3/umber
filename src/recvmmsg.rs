
use std::mem;

use libc::*;

const VLEN: c_uint = 10;
const BUFSIZE: c_uint = 200;
const TIMEOUT: c_long = 1;

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

    if bind(sockfd, &sa as *const _ as *const sockaddr, mem::size_of_val(&sa) as socklen_t) == -1 {
        panic!("bind()");
    }

    let mut msgs: [mmsghdr; VLEN as usize] = [ mem::zeroed(); VLEN as usize];
    let mut iovecs: [iovec; VLEN as usize] = [ mem::zeroed(); VLEN as usize];
    let mut bufs: [[c_char; VLEN as usize]; (BUFSIZE+1) as usize] = [[mem::zeroed(); VLEN as usize]; (BUFSIZE+1) as usize];
    for i in 0..VLEN as usize {
        iovecs[i].iov_base = bufs[i].as_mut_ptr() as *mut c_void;
        iovecs[i].iov_len = BUFSIZE as usize;
        msgs[i].msg_hdr.msg_iov = &mut iovecs[i] as *mut _ as *mut iovec;
        msgs[i].msg_hdr.msg_iovlen = 1;
    }

    let mut timeout = timespec {
        tv_sec: TIMEOUT,
        tv_nsec: 0,
    };

    let retval = recvmmsg(sockfd, msgs.as_mut_ptr(), VLEN, 0, &mut timeout as *mut timespec);
    if retval == -1 {
        panic!("recvmmsg()");
    }

    println!("{} messages received", retval);
    for i in 0..retval as usize {
        print!("{}", bufs[i].iter().map(|&s| s as u8 as char).collect::<String>());
    }

}

fn htonl(n: u32) -> u32 {
    n.to_be()
}

fn htons(n: u16) -> u16 {
    n.to_be()
}

