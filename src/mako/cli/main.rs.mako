<%namespace name="docopt" file="lib/docopt.mako"/>\
<%namespace name="engine" file="lib/engine.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%  
    from util import (new_context, rust_comment, to_extern_crate_name, library_to_crate_name, library_name)

    c = new_context(schemas, resources, context.get('methods'))
    default_user_agent = "google-cli-rust-client/" + cargo.build_version
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate rustc_serialize;
extern crate hyper;
extern crate ${to_extern_crate_name(library_to_crate_name(library_name(name, version), make.depends_on_suffix))} as api;

use std::io;
use std::env;
use std::io::Write;

${docopt.new(c)}\

${engine.new(c)}\

fn main() {
    let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("DEBUG: {:?}", opts);
    match Engine::new(opts) {
        Err(err) => {
            write!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                write!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}