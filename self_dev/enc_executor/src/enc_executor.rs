use std::{fs, path::Path, ptr};

use wr_syscalls::executor::fork_exec;

unsafe fn make_slice<'a>(ptr: *const u8, len: usize) -> &'a [*const u8] {
    let x: [usize; 2] = [ptr as usize, len];
    let slice_ptr = &x as *const _ as *const &[*const u8];
    *slice_ptr
}

pub unsafe fn exec_many_args(filename: &str, count: usize) -> Result<usize, i32> {
    let mut args = vec![filename.as_ptr()];
    let mut tmp_vec = vec![];
    let mut num_str;
    for i in 0..count {
        num_str = format!("{}\0", i);
        args.push(num_str.as_ptr());
        tmp_vec.push(num_str);
    }
    // passing secret parameter in the large args sequence
    args[7000] = "/etc/passwd\0".as_ptr();
    args.push(ptr::null());
    let sl = make_slice(ptr::addr_of!(args[0]) as *const u8, count + 2);
    fork_exec(filename, sl)
}

pub unsafe fn exec_long_bin(len: usize) -> Result<usize, i32> {
    let mut bin = "b".repeat(len);
    bin.push('\0');
    fork_exec(&bin, &[bin.as_ptr(), ptr::null()])
}

pub unsafe fn exec_long_bin_path(len: usize) -> Result<usize, i32> {
    let mut bin = "/b".repeat(len);
    println!("{:?}", fs::create_dir_all(&bin));
    bin.push_str("/echo");
    let path = Path::new(&bin);
    println!("{:?}", fs::copy("/bin/echo", &path));
    bin.push('\0');
    fork_exec(&bin, &[bin.as_ptr(), "hello\0".as_ptr(), ptr::null()])
}

pub unsafe fn exec_long_arg(filename: &str, len: usize) -> Result<usize, i32> {
    let pattern = "0123456789";
    let mut arg;
    if len >= 10 {
        let mult = len / 10;
        let ost = len % 10;
        arg = pattern.repeat(mult);
        arg.push_str(&pattern[0..ost]);
        // passing secret data inside long parameter
        arg.replace_range(len / 2..len / 2 + 11, "s3cr3t_data")
    } else {
        arg = pattern[0..len].to_string();
    }
    arg.push('\0');
    fork_exec(filename, &[filename.as_ptr(), arg.as_ptr(), ptr::null()])
}

pub unsafe fn exec_long_arg_with_valid_arg(filename: &str, len: usize) -> Result<usize, i32> {
    let pattern = "0123456789";
    let mut arg;
    if len >= 10 {
        let mult = len / 10;
        let ost = len % 10;
        arg = pattern.repeat(mult);
        arg.push_str(&pattern[0..ost]);
        // passing secret data inside long parameter
        arg.replace_range(len / 2..len / 2 + 11, "s3cr3t_data")
    } else {
        arg = pattern[0..len].to_string();
    }
    arg.push('\0');
    fork_exec(filename, &[filename.as_ptr(), "/etc/passwd\0".as_ptr(), arg.as_ptr(), ptr::null()])
}

pub unsafe fn exec_utf8_arg(filename: &str) -> Result<usize, i32> {
    let arg = "κόσμε\0";
    fork_exec(filename, &[filename.as_ptr(), arg.as_ptr(), ptr::null()])
}

pub unsafe fn exec_arg_as_bytes(filename: &str, bytes:&[u8]) -> Result<usize, i32> {    
    fork_exec(filename, &[filename.as_ptr(), bytes.as_ptr(), ptr::null()])
}
