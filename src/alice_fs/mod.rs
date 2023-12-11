//! Alice FS - file manager for alicedb project.
//! With this mod you can easly use functions for controlling your filesystem.
//!
//! TODO:
//!     [ ] - New names for functions.
//!     [ ] - Type for Result<Vec<PathBuf>, Box<dyn Error>>.
//!     [ ] - Comments for functions.
//!     [ ] - New function that writes data to last line.
//!     [ ] - Normal reading file.
//!
//! FIXME:
//!     [ ] - `write_into_files`.

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, Path};
use std::fs::{OpenOptions};
use std::io::{self, prelude::*, BufReader, SeekFrom};




pub fn create_dir(dirname: &str) -> Result<(), Box<dyn Error>> {
    match fs::create_dir(dirname) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Done!"),
    };
    Ok(())
}

pub fn list_dir(dirname: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();
    match fs::read_dir(dirname) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            files.push(path.unwrap().path());
        },
    };
    Ok(files)
}


pub fn create_file(filepath: &str) -> Result<File, Box<dyn Error>> {
    Ok(File::options().read(true).write(true).create_new(true).open(&filepath)?)
}



pub fn write_into_file(filepath: &str, data: String) {
    let k = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open(&filepath);
    match k {
        Ok(mut file) => match writeln!(file, "{}", data) {
            Err(e) => eprintln!("Couldn't write to file: {}", e),
            Ok(_) => println!("data has been written.")
        }
        Err(why) => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&filepath)
                .unwrap();

            if let Err(e) = writeln!(file, "{}", data) {
                eprintln!("Couldn't write to file: {}", e);
            };
        }
    }
}

pub fn into_field(path_to_table: String, field_name: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(&path_to_table)?;

    let mut reader = BufReader::new(&file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut lines: Vec<_> = contents.lines().map(|l| l.to_string()).collect();

    for line in &mut lines {
        if line.contains(field_name) {
            let k = format!("{},{}", line.replace("\n", ""), data);
            *line = k;
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(lines.join("\n").as_bytes())?;

    Ok(())
}

pub fn read_file(path_to_table: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).write(false).open(&path_to_table)?;
    let mut reader = BufReader::new(&file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut ret_lines: Vec<Vec<String>> = Vec::new();
    let mut lines: Vec<_> = contents.lines().map(|l| l.to_string()).collect();

    for i in &lines{
        let mut k = i.split(",").collect::<Vec<&str>>();
        let mut w = Vec::new();
        for j in k {
            w.push(j.to_string())
        }
        ret_lines.push(w);
    }
    Ok(ret_lines)

}

pub fn normal_read(path_to_file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(todo!())
}

pub fn parse_json_config<T>(path_to_config: &str) -> T {

}
