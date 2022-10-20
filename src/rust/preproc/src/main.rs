extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark::Parser;
use pulldown_cmark_to_cmark::fmt::cmark;
use std::io::{self, Read, Write};

fn main() {
    let md = {
        let mut buf = String::with_capacity(2048);
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let mut output = String::with_capacity(2048);
    cmark(
        Parser::new_ext(&md, pulldown_cmark::Options::all()).map(|e| {
            use pulldown_cmark::Event::*;
            match e {
                Start(ref tag) => {
                    use pulldown_cmark::Tag::*;
                    match tag {
                        CodeBlock(code) => Start(CodeBlock(format!("text{}", code).into())),
                        _ => e,
                    }
                }
                _ => e,
            }
        }),
        &mut output,
        None,
    )
    .unwrap();
    io::stdout().write_all(output.as_bytes()).unwrap();
}
