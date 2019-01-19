extern crate clap;
use clap::{App};

fn main() {
    App::new("SelfControl")
        .version("0.1")
        .author("Tigran Bayburtsyan <tigran@bayburtsyan.com>")
        .about("Helps to block websites that keeps sucking your time, like Facebook, Twitter etc...")
        .get_matches();
}
