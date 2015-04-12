<%namespace name="docopt" file="lib/docopt.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%  
    from util import (new_context, rust_comment)

    c = new_context(schemas, resources, context.get('methods'))
    default_user_agent = "google-cli-rust-client/" + cargo.build_version
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

${docopt.new(c)}\

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    println!("Hello, ${id} !");
}