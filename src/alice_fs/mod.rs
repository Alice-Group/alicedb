//! Alice FS - file manager for alicedb project.
//! With this mod you can easly use functions for controlling your filesystem.
//!
//! TODO:
//!     [ ] - New names for functions.
//!     [ ] - Type for Result<Vec<PathBuf>, Box<dyn Error>>.
//!     [+] - Comments for functions.
//!     [ ] - New function that writes data to last line.
//!     [ ] - Normal reading file.
//!     [ ] - convert parse_json_config into T.
//!
//! FIXME:
//!     [-] - `write_into_files` into engines.

use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};

use std::io::prelude::*;
use std::io::{self, prelude::*, BufReader, SeekFrom};

use std::path::PathBuf;

use serde::Deserialize;

use crate::configurator::Root;

// <---------- CODE ---------->


type Res = Result<(), Box<dyn Error>>;


// Creates directory.
// 
// Example:
// 
// ```rust
//      alice_fs.create_dir("~/.config/alicedb");
// ```
pub fn create_dir(dirpath: &str) -> Res {
    match fs::create_dir(dirpath) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Done"),
    }; 
    Ok(())
}

// List files in directory.
//
// Example:
// 
// ```rust
//      alice_fs.list_dir("~/.config/alicedb");
// ```
pub fn list_dir(dirpath: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();
    match fs::read_dir(dirpath) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            files.push(path.unwrap().path());
        },
    };
    Ok(files)
}

// Creates or re-creates file.
//
// Example:
//
// ```rust
//      alice_fs.create_file("~/.config/alicedb/data.txt");
// ```
pub fn create_file(filepath: &str) -> Result<File, Box<dyn Error>> {
    Ok(File::options().read(true).write(true).create_new(true).open(filepath)?)
}


// Simple read some file and returns String.
//
// Example
// ```rust
//      alice_fs.read("path/to/config.json")
// ```
pub fn read(filepath: &str) -> String {
    fs::read_to_string(filepath).expect("Cant read this file.")
}


// Deserialize Json file into struct.
pub fn parse_json_config(path_to_config: &str) -> Root {
    let mut config: Root = serde_json::from_str(&read(path_to_config)).unwrap();
    return config;
}
