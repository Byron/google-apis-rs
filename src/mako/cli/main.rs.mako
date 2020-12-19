<%namespace name="argparse" file="lib/argparse.mako"/>\
<%namespace name="engine" file="lib/engine.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%  
    from util import (new_context, rust_comment, to_extern_crate_name, library_to_crate_name, library_name,
                      indent_all_but_first_by)
    from cli import OUT_ARG, DEBUG_FLAG, opt_value

    c = new_context(schemas, resources, context.get('methods'))
    default_user_agent = "google-cli-rust-client/" + cargo.build_version
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate ${to_extern_crate_name(library_to_crate_name(library_name(name, version), make.depends_on_suffix))};

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use ${to_extern_crate_name(library_to_crate_name(library_name(name, version), make.depends_on_suffix))}::{api, Error};

mod client;

${engine.new(c)}\

fn main() {
    let mut exit_status = 0i32;
    ${argparse.new(c) | indent_all_but_first_by(1)}\
    let matches = app.get_matches();

    let debug = matches.is_present("${DEBUG_FLAG}");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
