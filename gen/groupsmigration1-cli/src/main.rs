// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

docopt!(Args derive Debug, "
Usage: 
    groupsmigration1 archive insert
    groupsmigration1 --help
");

fn main() {
    let _: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("Hello, groupsmigration:v1 !");
}