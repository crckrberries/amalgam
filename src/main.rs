use amalgam::*;
use regex::Regex;
use std::env::args;
use std::{env, fs};

fn main() {
    println!("[*] current directory: {:?}", env::current_dir().unwrap());
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("you need to supply args");
        return;
    }

    let pattern = &*args[1];
    let path = &*args[2];

    let paths = recurse(path, &Regex::new(&format!("(?i){}", pattern)).unwrap());
    let files: Vec<String> = paths
        .iter()
        .map(|f| {
            String::from_utf8(fs::read(f).unwrap()).unwrap_or_else(|e| {
                eprintln!(
                    "[-] {} is not utf-8 encoded; skipping",
                    f.file_name().unwrap().to_str().unwrap()
                );

                String::from("")
            })
        })
        .collect();

    let joined = files.join("\n");

    fs::write("amalgam.txt", joined).unwrap();
}
