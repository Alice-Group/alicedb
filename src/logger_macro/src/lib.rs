

use std::fmt::format;
use std::str::FromStr;
use std::string::ToString;
use chrono::Utc;
use std::time::SystemTime;
use chrono::DateTime;



extern crate proc_macro;
use proc_macro::TokenStream;



#[proc_macro_attribute]
pub fn loguru(attr: TokenStream, item: TokenStream) -> TokenStream {
    let now = Utc::now();
    println!("{:?}", attr);
    println!("Logged!");
    //write_into_file("log.txt", format!("[ UTC: {} ]>=====<[ {} ]>=====[ {}", now.format("%d-%b-%Y %H:%M:%S %P %z"), "Debug", "qwe"));
    item
}


#[cfg(test)]
mod tests {
    use super::*;

}
