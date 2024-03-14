#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::ffi::CStr;

extern "C" {
    pub fn inet_aton(cp: *const libc::c_char, inp: *mut libc::in_addr) -> libc::c_int;
    pub fn inet_addr(cp: *const libc::c_char) -> libc::in_addr_t;
    pub fn inet_ntoa(inp: libc::in_addr) -> *mut libc::c_char;
}

struct Udp_Client {
    fd : i32
}

impl Udp_Client {
    pub fn new() -> Udp_Client {
        Udp_Client {
            fd : -1
        }
    }

    pub fn get_fd(&self) -> i32 { return self.fd; }

    pub fn create(&mut self) -> i32 {
        unsafe {
            self.fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
            if self.fd < 0 {
                return -1
            }
        }

        return 0;
    }

    pub fn receive(&self, buf : &mut [u8], buf_len : usize, ipaddr : &mut String, port : &mut u32) -> i32 {
        let ret;
        unsafe {
            let mut sender_addr : libc::sockaddr_in = std::mem::zeroed();
            let mut len : libc::socklen_t = std::mem::size_of_val(&sender_addr) as u32;

            ret = libc::recvfrom(self.fd,
                                 buf as *mut [u8] as *mut libc::c_void,
                                 buf_len, 0,
                                 &mut sender_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                                 &mut len as *mut u32);
            if ret < 0 {
                return -1;
            }

            let sender_ipaddr = inet_ntoa(sender_addr.sin_addr);
            let c_str = CStr::from_ptr(sender_ipaddr);
            let str_slice = c_str.to_str().unwrap();
            *ipaddr = str_slice.to_owned();
            *port = sender_addr.sin_port.to_be() as u32;
        }

        return ret as i32;
    }

    pub fn send(&self, buf : &mut [u8], buf_len : usize, ipaddr : &String, port : u32) -> i32 {
        let ret : isize;

        unsafe {
            let mut dest_addr : libc::sockaddr_in = std::mem::zeroed();
            let len = std::mem::size_of_val(&dest_addr) as u32;

            dest_addr.sin_addr.s_addr = inet_addr(ipaddr.as_ptr() as *const libc::c_char);
            dest_addr.sin_port = port.to_be() as u16;
            dest_addr.sin_family = libc::AF_INET as u16;

            ret = libc::sendto(self.fd,
                               buf as *mut [u8] as *mut libc::c_void,
                               buf_len,
                               0,
                               &dest_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                               len);
        }

        return ret as i32;
    }

    pub fn delete(&self) {
        if self.fd > 0 {
            unsafe { libc::close(self.fd); }
        }
    }
}

