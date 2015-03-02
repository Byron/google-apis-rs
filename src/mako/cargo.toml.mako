<%! import util %>\
<%namespace name="mutil" file="lib/util.mako"/>\
# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !
[package]

name = "${mutil.library_name()}"
version = "${cargo.build_version}"
authors = [${",\n           ".join('"%s"' % a for a in cargo.authors)}]
description = "A complete library to interact with ${canonicalName} (protocol ${version})"
repository = "${mutil.repository_url()}"
homepage = "${documentationLink}"
documentation = "${cargo.doc_base_url}"
license = "${copyright.license_abbrev}"
keywords = ["${name}", ${", ".join(util.estr(cargo.keywords))}]

[dependencies]
# Just to get hyper to work !
openssl = "= 0.4.3"
# Just to get hyper to work !
cookie = "= 0.1.13"
hyper = "*"
rustc-serialize = "*"
yup-oauth2 = "*"

[dev-dependencies]
yup-hyper-mock = "*"
