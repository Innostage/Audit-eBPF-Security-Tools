use std::{env, ptr, thread};
use wr_syscalls::executor::fork_exec;

///
/// Ivan Gavrilov 2023
/// Researching for OFFZONE Conf
/// 

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args[1].clone();
    let step = args[2]
        .parse::<usize>()
        .expect("Expected number as the second arg");

    let filename: &'static str = "/bin/cat\0";    

    match mode.as_str() {
        "of" => unsafe {
            let to_read: &'static str = "/etc/passwd\0";
            let res = fork_exec_many_cpu_of(filename, to_read, step);
            println!("{:?}", res);
        },        
        "mf" => unsafe {
            let res = fork_exec_many_cpu_mf(filename, step);
            println!("{:?}", res);
        },
        _ => {
            println!("Expected mode: 'of' or 'mf'");
        }
    }
}

///
/// Executes programm on many files - 0, 1, 2, ... n
/// 
pub unsafe fn fork_exec_many_cpu_mf(filename: &'static str, step: usize) -> Result<usize, i32> {
    let core_ids = core_affinity::get_core_ids().unwrap();
    let handles = core_ids
        .into_iter()
        .map(|id| {
            thread::spawn(move || unsafe {
                let res = core_affinity::set_for_current(id);
                return if res {
                    for i in step * id.id..step * id.id + step {
                        fork_exec(
                            filename,
                            &[filename.as_ptr(), format!("{}\0", i).as_ptr(), ptr::null()],
                        )?;
                    }
                    Ok(0)
                } else {
                    Err(-45)
                };
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().expect("Couldn't join")?;
    }

    Ok(0)
}


///
/// Executes programm on one file
/// 
pub unsafe fn fork_exec_many_cpu_of(filename: &'static str, arg: &'static str, step: usize) -> Result<usize, i32> {
    

    let core_ids = core_affinity::get_core_ids().unwrap();
    let handles = core_ids
        .into_iter()
        .map(|id| {
            thread::spawn(move || unsafe {
                let res = core_affinity::set_for_current(id);
                return if res {
                    for _ in step * id.id..step * id.id + step {
                        fork_exec(
                            filename,
                            &[filename.as_ptr(), arg.as_ptr(), ptr::null()],
                        )?;
                    }
                    Ok(0)
                } else {
                    Err(-45)
                };
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().expect("Couldn't join")?;
    }

    Ok(0)
}
