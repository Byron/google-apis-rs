<%namespace name="argparse" file="lib/argparse.mako"/>\
<%namespace name="engine" file="lib/engine.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%  
    from util import (new_context, rust_comment, to_extern_crate_name, library_to_crate_name, library_name,
                      indent_all_but_first_by)

    c = new_context(schemas, resources, context.get('methods'))
    default_user_agent = "google-cli-rust-client/" + cargo.build_version
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate ${to_extern_crate_name(library_to_crate_name(library_name(name, version), make.depends_on_suffix))} as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

## ${engine.new(c)}\

fn main() {
    let matches = 
    ${argparse.new(c) | indent_all_but_first_by(1)}\

    ## let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    ## let debug = opts.flag_debug;
    ## match Engine::new(opts) {
    ##     Err(err) => {
    ##         writeln!(io::stderr(), "{}", err).ok();
    ##         env::set_exit_status(err.exit_code);
    ##     },
    ##     Ok(engine) => {
    ##         if let Some(err) = engine.doit() {
    ##             if debug {
    ##                 writeln!(io::stderr(), "{:?}", err).ok();
    ##             } else {
    ##                 writeln!(io::stderr(), "{}", err).ok();
    ##             }
    ##             env::set_exit_status(1);
    ##         }
    ##     }
    ## }
}