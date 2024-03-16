#![allow(dead_code)]
#![allow(non_camel_case_types)]

/**
 * @brief - Implements timestamp information.
 *
 * All are in UTC timestamp only.
 */
pub struct Timestamp {
    pub year    : u32,
    pub mon     : u32,
    pub day     : u32,
    pub hour    : u32,
    pub min     : u32,
    pub sec     : u32,
    pub nsec    : u64
}

impl Timestamp {
    /**
     * @brief - clears timestamp struct.
     */
    pub fn new() -> Timestamp {
        Timestamp {
            year    : 0,
            mon     : 0,
            day     : 0,
            hour    : 0,
            min     : 0,
            sec     : 0,
            nsec    : 0
        }
    }

    /**
     * @brief - get wallclock time
     */
    pub fn get_wallclock_time(ts : &mut Timestamp) -> i32 {
        let now : libc::time_t;
        let t : *mut libc::tm;

        unsafe {
            now = libc::time(0 as *mut i64);
            t = libc::gmtime(&now);
            if t == std::ptr::null_mut() {
                return -1;
            }

            let mut tp : libc::timespec = std::mem::zeroed();
            let ret = libc::clock_gettime(libc::CLOCK_REALTIME,
                                               &mut tp as *mut libc::timespec);
            if ret < 0 {
                return -1;
            }

            ts.year = (*t).tm_year as u32 + 1900;
            ts.mon = (*t).tm_mon as u32 + 1;
            ts.day = (*t).tm_mday as u32;
            ts.hour = (*t).tm_hour as u32;
            ts.min = (*t).tm_min as u32;
            ts.sec = (*t).tm_sec as u32;
            ts.nsec = tp.tv_nsec as u64;
        }

        return 0;
    }
}

pub fn get_sec() -> i64 {
    let now : libc::time_t;

    unsafe {
        now = libc::time(0 as *mut i64);
    }

    return now;
}

/**
 * @brief - defines the timeval
 */
pub struct Timeval {
    pub sec     : u32,
    pub nsec    : u64
}

impl Timeval {
    pub fn new() -> Timeval {
        Timeval {
            sec : 0,
            nsec : 0
        }
    }

    pub fn get_timeval(tv : &mut Timeval) -> i32 {
        let ret;

        unsafe {
            let mut tp : libc::timespec = std::mem::zeroed();

            ret = libc::clock_gettime(libc::CLOCK_REALTIME,
                                      &mut tp as *mut libc::timespec);
            if ret < 0 {
                return -1;
            }

            tv.sec = tp.tv_sec as u32;
            tv.nsec = tp.tv_nsec as u64;
        }

        return 0;
    }

    pub fn get_monotonic(tv : &mut Timeval) -> i32 {
        let ret;

        unsafe {
            let mut tp : libc::timespec = std::mem::zeroed();

            ret = libc::clock_gettime(libc::CLOCK_MONOTONIC,
                                      &mut tp as *mut libc::timespec);
            if ret < 0 {
                return -1;
            }

            tv.sec = tp.tv_sec as u32;
            tv.nsec = tp.tv_nsec as u64;
        }

        return 0;
    }
}

