use std::{
    process::exit,
    ptr,
};

use crate::wr_syscalls::{sys_execve, sys_fork};

pub unsafe fn fork() -> Result<usize, i32> {
    sys_fork().or_else(|op| Err(op.into_raw()))
}

pub unsafe fn execve(filename: *const u8, argv: *const u8, envp: *const u8) -> Result<usize, i32> {
    sys_execve(filename, argv, envp).or_else(|op| Err(op.into_raw()))
}

pub unsafe fn fork_exec(filename: &str, args: &[*const u8]) -> Result<usize, i32> {    
    let envp: [*const char; 1] = [ptr::null()];
    let child_id = fork()?;
    if child_id == 0 {
        execve(
            filename.as_ptr(),
            args.as_ptr() as *const u8,
            envp.as_ptr() as *const u8,
        )?;
        exit(0);
    }
    Ok(0)
}
