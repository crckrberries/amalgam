use regex::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub content: Vec<u8>,
}

pub fn recurse(path: impl AsRef<Path>, pattern: &Regex) -> Vec<PathBuf> {
    let mut buf = vec![];
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();

        if meta.is_dir() {
            let mut sub_dir = recurse(entry.path(), pattern);
            buf.append(&mut sub_dir);
        }

        if meta.is_file() && pattern.is_match(entry.file_name().to_str().unwrap()) {
            println!("[+] matched {}", entry.file_name().to_str().unwrap());
            buf.push(entry.path())
        }
    }

    buf
}
