use std::ffi::CString;

use libc::c_int;

const BYTES_PER_READ: usize = 50;

#[derive(Debug)]
pub enum FdError {
    OpenError,
    ReadError,
    WriteError,
}

impl std::fmt::Display for FdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FdError::OpenError => write!(f, "Failed to open file descriptor"),
            FdError::ReadError => write!(f, "Failed to read from file descriptor"),
            FdError::WriteError => write!(f, "Failed to write to file descriptor"),
        }
    }
}

type Result<T> = std::result::Result<T, FdError>;

#[derive(Debug)]
pub struct Fd {
    fd: i32,
}

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}

impl Fd {
    pub fn new(path: &str, mode: c_int) -> Result<Self> {
        let fd = unsafe { libc::open(CString::new(path).unwrap().as_ptr(), mode) };
        if fd < 0 {
            return Err(FdError::OpenError);
        }
        Ok(Self { fd })
    }

    // At least read min_len bytes, may read less if a '\n' is been read
    pub fn read(&self, min_len: usize) -> Result<String> {
        unsafe { libc::lseek(self.fd, 0, libc::SEEK_SET) };
        let mut buffer = [0u8; BYTES_PER_READ];
        let mut read_times = 0;
        let mut data = Vec::new();
        while read_times * BYTES_PER_READ < min_len {
            let ret = unsafe {
                libc::read(
                    self.fd,
                    &mut buffer as *mut _ as *mut libc::c_void,
                    BYTES_PER_READ,
                )
            };
            if ret < 0 {
                return Err(FdError::ReadError);
            } else if ret < BYTES_PER_READ as isize {
                // EOF
                data.extend_from_slice(&buffer[..ret as usize]);
                break;
            }
            read_times += 1;
            data.extend_from_slice(&buffer[..]);
        }
        let str = String::from_utf8_lossy(data.as_slice());
        Ok(str.trim().to_string())
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        todo!()
    }
}
