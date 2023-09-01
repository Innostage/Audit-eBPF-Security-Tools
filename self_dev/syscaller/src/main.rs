use std::{    
    io::Write,
    process::exit,
};

use wr_syscalls::{self, read_file::*, write_file::*, create_file::*};

fn ask_value(title: &str) -> String {
    print!("{} ", title);
    std::io::stdout().flush();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}

fn get_filename() -> String {
    let filename = ask_value("File:");
    return if filename.len() != 0 {
        format!("{}\0", filename)
    } else {
        String::from("/test/secret\0")
    };
}

unsafe fn handle_read_mode(mode: &str) {
    let res = match mode {
        "ro" => read_with_open(&get_filename()),
        "roa" => read_with_openat(&get_filename()),
        "pro" => pread_with_open(&get_filename(), 3),
        "rvo" => readv_with_open(&get_filename()),
        "rpvo" => preadv_with_open(&get_filename(), 3),
        _ => {
            println!("Unknown mode.");
            exit(1);
        }
    };
    println!("{:?}", res);
}

unsafe fn handle_write_mode(mode: &str) {
    let data = "s3cr3t\0";
    let res = match mode {
        "wo" => write_with_open(&get_filename(), data),
        "woa" => write_with_openat(&get_filename(), data),
        "wpo" => pwrite_with_open(&get_filename(), data, 3),
        "wvo" => writev_with_open(&get_filename(), &[data, data, data]),
        "wpvo" => pwritev_with_open(&get_filename(), &[data, data, data], 3),
        "wsfo" => sendfile_with_open(&get_filename(), &get_filename(), 3, 20),
        _ => {
            println!("Unknown mode.");
            exit(1);
        }
    };
    println!("{:?}", res);
}

unsafe fn handle_create_mode(mode:&str){
    let res = match mode{
        "co"=>create_with_open(&get_filename()),
        "cc"=>create_with_creat(&get_filename()),
        _ => {
            println!("Unknown mode.");
            exit(1);
        }
    };
    println!("{:?}", res);
}

fn main() {
    unsafe {
        let mode = ask_value("Mode:");

        match mode.chars().nth(0).unwrap() {
            'r' => {
                handle_read_mode(&mode);
            }
            'w' => {
                handle_write_mode(&mode);
            }
            'c' =>{
                handle_create_mode(&mode);
            }
            _ => {
                println!("Unknown mode.");
                exit(1);
            }
        }
    }
}
