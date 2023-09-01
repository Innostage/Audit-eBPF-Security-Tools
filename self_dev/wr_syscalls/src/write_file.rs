use std::ptr;

use syscalls::Errno;

use crate::{helper::openat, wr_syscalls::*};


pub unsafe fn write_with_open(file_path: &str, data: &str) -> Result<usize, Errno> {    
    let fd = sys_open(file_path.as_ptr(), 1)?;
    sys_write(fd, data.as_bytes().as_ptr(), data.len())?;
    sys_close(fd)
}

pub unsafe fn write_with_openat(file_path: &str, data: &str) -> Result<usize, Errno> {
    let (dirfd, filefd) = openat(file_path, 0, 1)?;
    sys_write(filefd, data.as_bytes().as_ptr(), data.len())?;
    sys_close(filefd)?;
    sys_close(dirfd)
}

pub unsafe fn pwrite_with_open(file_path: &str, data: &str, offset: usize) -> Result<usize, Errno> {
    let fd = sys_open(file_path.as_ptr(), 1)?;
    sys_pwrite(fd, data.as_bytes().as_ptr(), data.len(), offset)?;
    sys_close(fd)
}

#[derive(Debug)]
pub struct Iovec {
    iov_base: *const u8,
    iov_len: usize,
}

impl Iovec {
    pub fn from(data: &str) -> Self {
        Iovec {
            iov_base: data.as_ptr(),
            iov_len: data.len(),
        }
    }   

    pub fn from_buf(buf: &[u8]) -> Self {
        Iovec {
            iov_base: buf.as_ptr(),
            iov_len: buf.len(),
        }
    }  
}

pub unsafe fn writev_with_open(file_path: &str, data: &[&str; 3]) -> Result<usize, Errno> {
    let iovecs: [Iovec; 3] = data
        .iter()
        .map(|d| Iovec::from(d))
        .collect::<Vec<Iovec>>()
        .try_into()
        .expect("<Error while converting to an array>");
    let fd = sys_open(file_path.as_ptr(), 1)?;
    sys_writev(fd, iovecs.as_ptr() as *const u8, data.len())?;
    sys_close(fd)
}

pub unsafe fn pwritev_with_open(
    file_path: &str,
    data: &[&str; 3],
    offset: usize,
) -> Result<usize, Errno> {
    let iovecs: [Iovec; 3] = data
        .iter()
        .map(|d| Iovec::from(d))
        .collect::<Vec<Iovec>>()
        .try_into()
        .expect("<Error while converting to an array>");
    let fd = sys_open(file_path.as_ptr(), 1)?;
    sys_pwritev(fd, iovecs.as_ptr() as *const u8, data.len(), offset)?;
    sys_close(fd)
}


pub unsafe fn sendfile_with_open(in_file_path: &str, out_file_path:&str, offset:usize, count:usize)-> Result<usize, Errno>{

    let in_fd = sys_open(in_file_path.as_ptr(), 0)?;
    let out_fd = sys_open(out_file_path.as_ptr(), 1)?;
    println!("SendingFile");
    sys_sendfile(out_fd, in_fd, ptr::null(), count)
}