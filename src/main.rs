#![allow(warnings)]

use crate::conductor::Conductor;

mod conductor;

mod commit_manager;
mod backer;
mod executor;
mod alice_fs;
mod cacher;
mod compression;

use compression::*;
use executor::*;
use commit_manager::*;
use crate::conductor::transaction::Transaction;
use cacher::*;

fn main() {
    //create_commits_file();
    let mut cacher = Cacher::init();
    cacher.insert( Response { request_type: Actions::SelectAll, response: vec!["Hello_world".into()] });
    cacher.print();
    commit!(create_transaction!("qwe", "rer", "123"));
}
