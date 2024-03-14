#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::ffi::CStr;

extern "C" {
    pub fn inet_aton(cp: *const libc::c_char, inp: *mut libc::in_addr) -> libc::c_int;
    pub fn inet_addr(cp: *const libc::c_char) -> libc::in_addr_t;
    pub fn inet_ntoa(inp: libc::in_addr) -> *mut libc::c_char;
}


struct Tcp_Server {
    fd : i32
}

impl Tcp_Server {
    pub fn new() -> Tcp_Server {
        Tcp_Server {
            fd : -1
        }
    }

    pub fn get_fd(&self) -> i32 { return self.fd; }

    pub fn create(&mut self, ipaddr : &String, port : u32, n_connections : i32) -> i32 {
        let mut ret;

        unsafe {
            self.fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if self.fd < 0 {
                return -1;
            }
        }

        unsafe {
            let reuse : u32 = 1;

            ret = libc::setsockopt(self.fd,
                                   libc::SOL_SOCKET,
                                   libc::SO_REUSEADDR,
                                   &reuse as *const u32 as *const libc::c_void,
                                   4);
            if ret < 0 {
                libc::close(self.fd);
                return -1;
            }

            let mut serv_addr : libc::sockaddr_in = std::mem::zeroed();
            let serv_addr_len = std::mem::size_of_val(&mut serv_addr);

            serv_addr.sin_addr.s_addr = inet_addr(ipaddr.as_ptr() as *const libc::c_char);
            serv_addr.sin_port = port.to_be() as u16;

            ret = libc::bind(self.fd,
                             &serv_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                             serv_addr_len as u32);
            if ret < 0 {
                libc::close(self.fd);
                self.fd = -1;
                return -1;
            }

            ret = libc::listen(self.fd, n_connections);
            if ret < 0 {
                libc::close(self.fd);
                return -1;
            }
        }

        return 0;
    }

    pub fn accept(&self, ipaddr : &mut String, port : &mut u32) -> i32 {
        let client;

        unsafe {
            let mut client_addr : libc::sockaddr_in = std::mem::zeroed();
            let mut client_addr_len : libc::socklen_t = std::mem::size_of_val(&client_addr) as u32;
            client = libc::accept(self.fd,
                                  &mut client_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                                  &mut client_addr_len as *mut u32);
            if client < 0 {
                return -1;
            }

            let ipaddr_str = inet_ntoa(client_addr.sin_addr);
            let c_str = CStr::from_ptr(ipaddr_str);
            let str_slice = c_str.to_str().unwrap();
            *ipaddr = str_slice.to_owned();
            *port = client_addr.sin_port.to_be() as u32;
        }

        return client;
    }

    pub fn receive(fd : i32, buf : &mut [u8], buf_len : usize) -> isize {
        let ret;

        unsafe {
            ret = libc::read(fd, buf as *mut [u8] as *mut libc::c_void, buf_len);
        }
    
        return ret;
    }

    pub fn send(fd : i32, buf : &mut [u8], buf_len : usize) -> isize {
        let ret;

        unsafe {
            ret = libc::write(fd, buf as *const [u8] as *const libc::c_void, buf_len);
        }

        return ret;
    }

    pub fn delete(&self) {
        if self.fd > 0 {
            unsafe { libc::close(self.fd); }
        }
    }

    pub fn delete_client(fd : i32) {
        if fd > 0 {
            unsafe { libc::close(fd); }
        }
    }
}

