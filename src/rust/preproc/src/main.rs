extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark::{CowStr, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use std::io::{self, Read, Write};

fn main() {
    let md = {
        let mut buf = String::with_capacity(2048);
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let mut output = String::with_capacity(2048);
    let url_base = std::env::var("URL_BASE").unwrap_or(String::new());
    // FIXME: for urls starting with /, use only the netloc
    let url_base = url_base
        .strip_suffix("/")
        .map(String::from)
        .unwrap_or(url_base);

    fn fix_url<'a>(base: &str, url: CowStr<'a>) -> CowStr<'a> {
        if url.starts_with("/") {
            format!("{base}{url}").into()
        } else if url.starts_with("..") {
            format!("{base}/{url}").into()
        } else {
            url
        }
    }

    cmark(
        Parser::new_ext(&md, pulldown_cmark::Options::all()).map(|e| {
            use pulldown_cmark::Event::*;
            match e {
                Start(ref tag) => {
                    use pulldown_cmark::Tag::*;
                    match tag {
                        CodeBlock(pulldown_cmark::CodeBlockKind::Indented) => Start(CodeBlock(
                            pulldown_cmark::CodeBlockKind::Fenced("text".into()),
                        )),
                        Link(lt, url, title) => {
                            Start(Link(*lt, fix_url(&url_base, url.clone()), title.clone()))
                        }
                        _ => e,
                    }
                }
                End(Tag::Link(lt, url, title)) => {
                    End(Tag::Link(lt, fix_url(&url_base, url), title))
                }
                _ => e,
            }
        }),
        &mut output,
    )
    .unwrap();
    io::stdout().write_all(output.as_bytes()).unwrap();
}
