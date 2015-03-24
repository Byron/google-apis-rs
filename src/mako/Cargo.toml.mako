<%! from util import (estr, hash_comment) %>\
<%namespace name="util" file="lib/util.mako"/>\
<%block filter="hash_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
[package]

name = "${util.crate_name()}"
version = "${util.crate_version()}"
authors = [${",\n           ".join('"%s"' % a for a in cargo.authors)}]
description = "A complete library to interact with ${util.canonical_name()} (protocol ${version})"
repository = "${util.github_source_root_url()}"
% if documentationLink is not UNDEFINED:
homepage = "${documentationLink}"
% endif
documentation = "${cargo.doc_base_url}/${util.crate_name()}"
license = "${copyright.license_abbrev}"
keywords = ["${name}", ${", ".join(estr(cargo.keywords))}]

[dependencies]
hyper = "*"
mime = "*"
yup-oauth2 = "*"
% for dep in cargo.dependencies:
${dep}
% endfor
