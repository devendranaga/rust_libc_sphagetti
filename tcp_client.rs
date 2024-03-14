#![allow(dead_code)]
#![allow(non_camel_case_types)]


extern "C" {
    pub fn inet_aton(cp: *const libc::c_char, inp: *mut libc::in_addr) -> libc::c_int;
    pub fn inet_addr(cp: *const libc::c_char) -> libc::in_addr_t;
    pub fn inet_ntoa(inp: libc::in_addr) -> *mut libc::c_char;
}


struct Tcp_Client {
    fd : i32
}

impl Tcp_Client {
    pub fn new() -> Tcp_Client {
        Tcp_Client {
            fd : -1
        }
    }

    pub fn create(&mut self, ipaddr : &String, port : u32) -> i32 {
        let ret;

        unsafe {
            self.fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if self.fd < 0 {
                return -1;
            }
        }

        unsafe {
            let mut serv_addr : libc::sockaddr_in = std::mem::zeroed();
            let serv_addr_len = std::mem::size_of_val(&mut serv_addr);

            serv_addr.sin_addr.s_addr = inet_addr(ipaddr.as_ptr() as *const libc::c_char);
            serv_addr.sin_port = port.to_be() as u16;

            ret = libc::connect(self.fd,
                                &serv_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                                serv_addr_len as u32);
            if ret < 0 {
                libc::close(self.fd);
                self.fd = -1;
                return -1;
            }
        }

        return 0;
    }

    pub fn receive(&self, buf : &mut [u8], buf_len : usize) -> isize {
        let ret;

        unsafe {
            ret = libc::read(self.fd, buf as *mut [u8] as *mut libc::c_void, buf_len);
        }
    
        return ret;
    }

    pub fn send(&self, buf : &mut [u8], buf_len : usize) -> isize {
        let ret;

        unsafe {
            ret = libc::write(self.fd, buf as *const [u8] as *const libc::c_void, buf_len);
        }

        return ret;
    }

    pub fn delete(&self) {
        if self.fd > 0 {
            unsafe { libc::close(self.fd); }
        }
    }
}

