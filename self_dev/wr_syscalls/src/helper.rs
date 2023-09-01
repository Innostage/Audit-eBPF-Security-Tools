use std::path::Path;

use syscalls::Errno;

use crate::wr_syscalls::{sys_openat, sys_open};

pub fn get_parent_dir(file_path:&Path)->String{
    format!("{}\0",file_path.parent().expect("<Invalid filename>").to_str().expect("<Invalid filename>"))
}

pub fn get_filename(file_path:&Path)->String{
    format!("{}\0",file_path.file_name().expect("<Invalid filename>").to_str().expect("<Invalid filename>"))
}

pub unsafe fn openat(file_path:&str, dir_flags:usize, file_flags: usize)-> Result<(usize, usize), Errno>{
    let path = Path::new(file_path);
    let dir = get_parent_dir(path);
    let filename = get_filename(path);
    let dirfd = sys_open(dir.as_ptr(), dir_flags)?;
    let filefd = sys_openat(dirfd, filename.as_ptr(), file_flags)?;
    Ok((dirfd, filefd))
}