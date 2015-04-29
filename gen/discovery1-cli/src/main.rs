// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_discovery1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

fn main() {
    let matches = 
    App::new("discovery1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0")
           .about("Lets you discover information about other Google APIs, such as what APIs are available, the resource and method details for each API.")
           .after_help("All documentation details can be found athttp://byron.github.io/google-apis-rs/google_discovery1_cli/index.html")
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ${CONFIG_DIR}]")
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .takes_value(false))
           .subcommand(
               SubCommand::new("apis")
               .subcommand(
                   SubCommand::new("get-rest")
                       .about("Retrieve the description of a particular version of an api.")
                       .arg(
                           Arg::with_name("api")
                                   .help("The name of the API.")
                                   .required(true)
                                   .multiple(false))
                       .arg(
                           Arg::with_name("version")
                                   .help("The version of the API.")
                                   .required(true)
                                   .multiple(false))
                       .arg(
                           Arg::with_name("v")
                                   .short("p")
                                   .help("Set various fields of the request structure")
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(true))
                       .arg(
                           Arg::with_name("<out>")
                                   .short("o")
                                   .help("Specify the file into which to write the programs output")
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false))
                   )
               .subcommand(
                   SubCommand::new("list")
                       .about("Retrieve the list of APIs supported at this endpoint.")
                       .arg(
                           Arg::with_name("v")
                                   .short("p")
                                   .help("Set various fields of the request structure")
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(true))
                       .arg(
                           Arg::with_name("<out>")
                                   .short("o")
                                   .help("Specify the file into which to write the programs output")
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false))
                   )
               )
           .get_matches();
           
    
}