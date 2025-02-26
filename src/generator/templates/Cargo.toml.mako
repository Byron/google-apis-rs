<%! from generator.lib.util import (estr, enclose_in, hash_comment, library_to_crate_name, to_extern_crate_name) %>\
<%namespace name="util" file="../lib/util.mako"/>\
<%block filter="hash_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
[package]

name = "${util.crate_name()}"
version = "${util.crate_version()}"
authors = [${",\n           ".join('"%s"' % a for a in cargo.authors)}]
description = "A complete library to interact with ${util.canonical_name()} (protocol ${version})"
repository = "${util.github_source_root_url()}"
% if documentationLink is not UNDEFINED and documentationLink:
homepage = "${documentationLink}"
% endif
documentation = "${util.doc_base_url()}"
license = "${copyright.license_abbrev}"
keywords = ["${name[:20]}", ${", ".join(estr(cargo.keywords))}]
autobins = false
edition = "2021"

% if cargo.get('is_executable'):
[[bin]]
name = "${util.program_name()}"
path = "src/main.rs"
% endif

[dependencies]
chrono = { version = "0.4", default-features = false, features = ["clock"] }
% if cargo.get('is_executable'):
clap = "2"
http-body-util = "0.1"
% endif
hyper = "1"
hyper-rustls = { version = "0.27", default-features = false, features = ["http2", "rustls-native-certs", "native-tokio"] }
hyper-util = "0.1"
mime = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
% if not cargo.get('is_executable'):
serde_with = "3"
% endif
% if not cargo.get('is_executable'):
tokio = "1"
% else:
tokio = { version = "1", features = ["full"] }
% endif
url = "2"
utoipa = { version = "4", optional = true }
yup-oauth2 = { version = "12", default-features = false, optional = true }

google-apis-common = { path = "../../google-apis-common", version = "8" }
% if cargo.get('is_executable'):
google-clis-common = { path = "../../google-clis-common", version = "8" }
% endif

<%
  api_name = util.library_name()
  crate_name_we_depend_on = None

  if make.depends_on_suffix is not None:
    crate_name_we_depend_on = library_to_crate_name(api_name, suffix=make.depends_on_suffix)
%>\

% if make.depends_on_suffix is not None:

[dependencies.${crate_name_we_depend_on}]
path = "../${api_name}"
version = "${util.crate_version()}"
% endif

% if not cargo.get('is_executable'):
[features]
default = ["yup-oauth2", "ring"]
utoipa = ["dep:utoipa"]

## Enable OAuth 2.0 authentication support via the `yup-oauth2` crate
yup-oauth2 = ["dep:yup-oauth2", "google-apis-common/yup-oauth2"]

## Enable Service Account support for the `yup-oauth2 crate
yup-oauth2-service-account = ["yup-oauth2", "yup-oauth2/service-account", "google-apis-common/yup-oauth2-service-account"]

## Use AWS-LC as the crypto backend
##
## Either this feature or `ring` must be enabled when enabling `yup-oauth2-service-account`
aws-lc-rs = ["yup-oauth2?/aws-lc-rs", "google-apis-common/aws-lc-rs", "hyper-rustls/aws-lc-rs"]

## Use Ring as the crypto backend
##
## Either this feature or `aws-lc-rs` must be enabled when enabling `yup-oauth2-service-account`
ring = ["yup-oauth2?/ring", "google-apis-common/ring", "hyper-rustls/ring"]
% endif
