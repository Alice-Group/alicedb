#![allow(warnings)]

use crate::conductor::Conductor;

mod conductor;

mod commit_manager;
mod backer;
mod executor;
mod alice_fs;
mod compression;

use compression::*;
use executor::*;
use commit_manager::*;
use crate::conductor::transaction::Transaction;
use cacher::Cacher;


fn main() {
    print_ascii_banner();
}
