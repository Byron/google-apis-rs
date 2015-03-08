<%! from util import (estr, hash_comment) %>\
<%namespace name="util" file="lib/util.mako"/>\
<%block filter="hash_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
[package]

name = "${util.library_name()}"
version = "${cargo.build_version}"
authors = [${",\n           ".join('"%s"' % a for a in cargo.authors)}]
description = "A complete library to interact with ${util.canonical_name()} (protocol ${version})"
repository = "${util.repository_url()}"
homepage = "${documentationLink}"
documentation = "${cargo.doc_base_url}"
license = "${copyright.license_abbrev}"
keywords = ["${name}", ${", ".join(estr(cargo.keywords))}]

[dependencies]
hyper = "*"
rustc-serialize = "*"
yup-oauth2 = "*"

[dev-dependencies]
yup-hyper-mock = "*"
