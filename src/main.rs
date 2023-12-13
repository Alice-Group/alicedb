#![allow(warnings)]

mod alice_fs;
mod configurator;
mod engines;

use alice_fs::*;
use configurator::*;

use std::env;

use journalist::log;

use engines::*;



fn print_ascii_banner() {
    println!("

    ██   █    ▄█ ▄█▄    ▄███▄   ██▄   ███   
    █ █  █    ██ █▀ ▀▄  █▀   ▀  █  █  █  █  
    █▄▄█ █    ██ █   ▀  ██▄▄    █   █ █ ▀ ▄ 
    █  █ ███▄ ▐█ █▄  ▄▀ █▄   ▄▀ █  █  █  ▄▀ 
    █     ▀ ▐ ▀███▀  ▀███▀   ███▀  ███   
    █                                     
    ▀                                      
    ");
}


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
