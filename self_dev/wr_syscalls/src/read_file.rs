use std::{str::from_utf8, ffi::{c_uchar, c_int, c_uint}, ptr::addr_of};

use syscalls::Errno;

use crate::{wr_syscalls::*, helper::openat, write_file::Iovec};

unsafe fn read(fd: usize) -> Result<[u8; 20], Errno> {
    let buf = [0; 20];
    sys_read(fd, buf.as_ptr(), 20)?;
    Ok(buf)
}

unsafe fn pread(fd: usize, offset: usize) -> Result<[u8; 20], Errno> {
    let buf = [0; 20];
    sys_pread(fd, buf.as_ptr(), 20, offset)?;
    Ok(buf)
}

pub unsafe fn read_with_open(file_path:&str)-> Result<String, Errno>{
    let fd = sys_open(file_path.as_ptr(), 0)?;
    let bytes = read(fd)?;    
    sys_close(fd)?;
    Ok(from_utf8(&bytes).expect("<Invalid utf8 characters in file>").to_string())
}

pub unsafe fn read_with_openat(file_path:&str)-> Result<String, Errno>{
    let (dirfd, filefd) = openat(file_path, 0, 0)?;
    let bytes = read(filefd)?; 
    sys_close(filefd)?;
    sys_close(dirfd)?;    
    Ok(from_utf8(&bytes).expect("<Invalid utf8 characters in file>").to_string())
}

pub unsafe fn pread_with_open(file_path:&str, offset: usize)-> Result<String, Errno>{
    let fd = sys_open(file_path.as_ptr(), 0)?;
    let bytes = pread(fd, offset)?;    
    sys_close(fd)?;
    Ok(from_utf8(&bytes).expect("<Invalid utf8 characters in file>").to_string())
}

pub unsafe fn readv_with_open(file_path:&str)-> Result<String, Errno>{
    let bufs = [[0; 3],[0;3],[0;3]];
    let iovecs = [Iovec::from_buf(&bufs[0]),Iovec::from_buf(&bufs[1]), Iovec::from_buf(&bufs[2])];
    let fd = sys_open(file_path.as_ptr(), 0)?;
    sys_readv(fd, iovecs.as_ptr() as *const u8, iovecs.len())?;    
    sys_close(fd)?;
    let data = bufs.iter().map(|b| from_utf8(b).expect("<Invalid utf-8 data>")).collect::<Vec<&str>>().join("");
    Ok(data)
}

pub unsafe fn preadv_with_open(file_path:&str, offset:usize)-> Result<String, Errno>{
    let bufs = [[0; 3],[0;3],[0;3]];
    let iovecs = [Iovec::from_buf(&bufs[0]),Iovec::from_buf(&bufs[1]), Iovec::from_buf(&bufs[2])];
    let fd = sys_open(file_path.as_ptr(), 0)?;
    sys_preadv(fd, iovecs.as_ptr() as *const u8, iovecs.len(), offset)?;    
    sys_close(fd)?;
    let data = bufs.iter().map(|b| from_utf8(b).expect("<Invalid utf-8 data>")).collect::<Vec<&str>>().join("");
    Ok(data)
}