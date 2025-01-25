#![allow(unused)]

use std::{ffi::CString, mem::MaybeUninit};

use crate::lib::c_lib::c_strcpy;

pub struct tap_device {
    fd : i32
}

impl tap_device {
    pub fn new() -> tap_device {
        let tap = tap_device {
            fd : -1
        };
        tap
    }

    pub fn create(&mut self, devname : &String) -> i32 {
        const TUN_DEV : &str = "/dev/net/tun";
        let tun_dev = CString::new(TUN_DEV).unwrap();
        let tun_dev_c_str : *const libc::c_char = tun_dev.as_ptr();

        unsafe {
            self.fd = libc::open(tun_dev_c_str as *const i8, libc::O_RDWR);
            if self.fd < 0 {
                log::error!("failed to open tun dev");
                libc::perror("test".as_ptr() as *const libc::c_char);
                return -1;
            }

            let mut req : libc::ifreq = MaybeUninit::zeroed().assume_init();

            req.ifr_ifru.ifru_flags = libc::IFF_TAP as i16;
            c_strcpy::c_strcpy(&mut req.ifr_name, devname.as_str());

            let mut ret = libc::ioctl(self.fd, libc::TUNSETIFF, &req);
            if ret < 0 {
                libc::close(self.fd);
                return -1;
            }

            ret = libc::ioctl(self.fd, libc::SIOCGIFFLAGS, &req);
            if ret < 0 {
                libc::close(self.fd);
                return -1;
            }

            req.ifr_ifru.ifru_flags |= libc::IFF_UP as i16;
            ret = libc::ioctl(self.fd, libc::SIOCSIFFLAGS, &req);
            if ret < 0 {
                libc::close(self.fd);
                return -1;
            }
        }

        return 0;
    }
}
