

use syscalls::Errno;

use crate::wr_syscalls::{sys_close, sys_creat, sys_open};

pub unsafe fn create_with_open(file_path: &str) -> Result<usize, Errno> {
    let filefd = sys_open(file_path.as_ptr(), 100)?;
    sys_close(filefd)
}

pub unsafe fn create_with_creat(file_path: &str) -> Result<usize, Errno> {
    let filefd = sys_creat(file_path.as_ptr())?;
    sys_close(filefd)
}
