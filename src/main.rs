#![allow(warnings)]

use crate::conductor::Conductor;

mod conductor;

mod commit_manager;
mod backer;
mod executor;
mod alice_fs;
mod compression;
mod misc;
mod configurator; 

use configurator::*;
use misc::*;
use compression::*;
use executor::*;
use commit_manager::*;
use crate::conductor::transaction::Transaction;
use cacher::Cacher;
use journalist::*;
use alice_fs::*;

use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        log("ERROR", "No settings file specified!");
        return;
    }

    let config_path = &args[1];

    print_ascii_banner();
    log("DONE", "Database started sucessfully!");
    log("OK", "AliceDB config specified!");
    let data: Root = parse_json_config(config_path);
    log("DONE", "Config successfuly readed and parsed!");

}
