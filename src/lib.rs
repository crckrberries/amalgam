use regex::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

// recursively gets all the files in a directory and its subdirectories
pub fn recurse(path: impl AsRef<Path>, pattern: &Regex) -> Vec<PathBuf> {
    let mut buf = vec![];
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();

        if meta.is_dir() {
            let mut sub_dir = recurse(entry.path(), pattern); // recursion!
            buf.append(&mut sub_dir);
        }

        if meta.is_file() && pattern.is_match(entry.file_name().to_str().unwrap()) {
            println!("[+] matched {}", entry.file_name().to_str().unwrap()); // guh
            buf.push(entry.path())
        }
    }

    buf
}
