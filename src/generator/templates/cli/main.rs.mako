<%namespace name="argparse" file="lib/argparse.mako"/>\
<%namespace name="engine" file="lib/engine.mako"/>\
<%namespace name="util" file="../../lib/util.mako"/>\
<%  
    from generator.lib.util import (new_context, rust_comment, to_extern_crate_name, library_to_crate_name, library_name,
                      indent_all_but_first_by)
    from generator.lib.cli import OUT_ARG, DEBUG_FLAG, opt_value

    c = new_context(schemas, resources)
    default_user_agent = "google-cli-rust-client/" + cargo.build_version
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use ${to_extern_crate_name(library_to_crate_name(library_name(name, version), make.depends_on_suffix))}::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

${engine.new(c)}\

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    ${argparse.new(c) | indent_all_but_first_by(1)}\
    let matches = app.get_matches();

    let debug = matches.is_present("a${DEBUG_FLAG}");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
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
