<%namespace name="docopt" file="lib/docopt.mako"/>\
<%namespace name="engine" file="lib/engine.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%  
    from util import (new_context, rust_comment)

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

use std::io;
use std::env;
use std::io::Write;

${docopt.new(c)}\

${engine.new(c)}\

fn main() {
    let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("{:?}", opts);
    match Engine::new(opts) {
        Err(e) => {
            write!(io::stderr(), "{:?}", e).ok();
            env::set_exit_status(e.exit_code);
        },
        Ok(mut engine) => {

        }
    }
}