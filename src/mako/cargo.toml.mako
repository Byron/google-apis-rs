# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !
[package]

name = "${name}${version[1:]}"
version = "${cargo.build_version}"
authors = [${",\n           ".join('"%s"' % a for a in cargo.authors)}]
description = "A library to interact with ${canonicalName} (protocol ${version})"
repository = "${cargo.repo_base_url}/${OUTPUT_DIR}"
homepage = "${documentationLink}"
documentation = "${cargo.doc_base_url}"
license = "MIT"
keywords = ["${name}", ${", ".join('"%s"' % k for k in cargo.keywords)}]

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
