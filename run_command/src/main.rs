extern crate clap;

use std::process::Command;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Generic Command Runner")
                      .version("0.0.1")
                      .author("Fletcher Haynes <fletcher@subnetzero.io>")
                      .about("Executes one command passed in as an argument")
                      .arg(Arg::with_name("INPUT")
                           .help("Sets the input file to use")
                           .required(true)
                           .index(1))
                      .get_matches();

    println!("Would execute: {}", matches.value_of("INPUT").unwrap());

}
