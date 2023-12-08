#![allow(warnings)]

use crate::conductor::Conductor;

mod conductor;

mod commit_manager;
mod backer;
mod executor;
mod alice_fs;
mod cache;

use commit_manager::*;
use crate::conductor::transaction::Transaction;


fn main() {
    //create_commits_file();
    commit!(create_transaction!("qwe", "rer", "123"));


}
