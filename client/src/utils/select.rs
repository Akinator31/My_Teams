use std::os::unix::io::RawFd;

const FD_SETSIZE: usize = 1024;
const NFDBITS: usize = 64;

#[repr(C)]
pub struct FdSet {
    fds_bits: [u64; FD_SETSIZE / NFDBITS],
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

unsafe extern "C" {
    fn select(
        nfds: i32,
        readfds: *mut FdSet,
        writefds: *mut FdSet,
        exceptfds: *mut FdSet,
        timeout: *mut Timeval,
    ) -> i32;
}

impl FdSet {
    pub fn new() -> Self {
        Self {
            fds_bits: [0; FD_SETSIZE / NFDBITS],
        }
    }

    pub fn zero(&mut self) {
        for i in 0..self.fds_bits.len() {
            self.fds_bits[i] = 0;
        }
    }

    pub fn set(&mut self, fd: RawFd) {
        let fd = fd as usize;
        self.fds_bits[fd / NFDBITS] |= 1 << (fd % NFDBITS);
    }

    pub fn is_set(&self, fd: RawFd) -> bool {
        let fd = fd as usize;
        (self.fds_bits[fd / NFDBITS] & (1 << (fd % NFDBITS))) != 0
    }
}

pub fn fd_select(
    nfds: i32,
    readfds: Option<&mut FdSet>,
    writefds: Option<&mut FdSet>,
    exceptfds: Option<&mut FdSet>,
    timeout: Option<&mut Timeval>,
) -> i32 {
    unsafe {
        select(
            nfds,
            readfds.map_or(std::ptr::null_mut(), |r| r),
            writefds.map_or(std::ptr::null_mut(), |w| w),
            exceptfds.map_or(std::ptr::null_mut(), |e| e),
            timeout.map_or(std::ptr::null_mut(), |t| t),
        )
    }
}
