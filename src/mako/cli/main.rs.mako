<%namespace name="docopt" file="lib/docopt.mako"/>\
<%  
    from util import new_context

    c = new_context(schemas, resources, context.get('methods'))
    default_user_agent = "google-cli-rust-client/" + cargo.build_version
%>\
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

<%docopt:new c="c"/>\

fn main() {
    let _: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("Hello, ${id} !");
}