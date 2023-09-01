use std::{env, process::exit};

use enc_executor::exec_many_args;

use crate::enc_executor::{
    exec_arg_as_bytes, exec_long_arg, exec_long_arg_with_valid_arg, exec_long_bin,
    exec_long_bin_path, exec_utf8_arg,
};

pub mod enc_executor;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        exit(-1);
    }
    unsafe {
        let res = match args[1].as_str().trim() {
            "ma" => {
                let sec = args[2]
                    .parse::<usize>()
                    .expect("Expected number as the second arg");
                exec_many_args("/bin/cat\0", sec)
            }
            "la" => {
                let sec = args[2]
                    .parse::<usize>()
                    .expect("Expected number as the second arg");
                match args[3].as_str() {
                    "nr" => exec_long_arg("/bin/echo\0", sec),
                    "rd" => exec_long_arg_with_valid_arg("/bin/cat\0", sec),
                    _ => {
                        exit(1);
                    }
                }
            }
            "lb" => {
                let sec = args[2]
                    .parse::<usize>()
                    .expect("Expected number as the second arg");
                exec_long_bin(sec)
            }
            "lbp" => {
                let sec = args[2]
                    .parse::<usize>()
                    .expect("Expected number as the second arg");
                exec_long_bin_path(sec)
            }
            "u8" => exec_utf8_arg("/bin/echo\0"),
            "ab" => {
                let bytes: &[u8] = &args
                    .iter()
                    .skip(2)
                    .map(|a| u8::from_str_radix(a, 16).expect("expected byte"))
                    .collect::<Vec<u8>>();
                exec_arg_as_bytes("/bin/echo\0", bytes)
            }
            _ => {
                println!("Unknown command. Available commands: ma, la, lb, lbp, u8, ab.");
                exit(1);
            }
        };
        println!("{:?}", res);
    }
}
