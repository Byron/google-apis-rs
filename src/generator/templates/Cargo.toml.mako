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
edition = "2018"

% if cargo.get('is_executable', False):
[[bin]]
name = "${util.program_name()}"
path = "src/main.rs"
% endif

[dependencies]
anyhow = "^ 1.0"
hyper-rustls = "0.24.0"
## Must match the one hyper uses, otherwise there are duplicate similarly named `Mime` structs
mime = "^ 0.3.0"
serde = { version = "^ 1.0", features = ["derive"] }
serde_json = "^ 1.0"
itertools = "^ 0.10"
% if cargo.get('is_executable', False):
google-clis-common = { path = "../../google-clis-common", version = "6.0" }
% else:
google-apis-common = { path = "../../google-apis-common", version = "6.0" }
% endif
% for dep in cargo.get('dependencies', list()):
${dep}
% endfor

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

% if not cargo.get("is_executable", False):
[features]
yup-oauth2 = ["google-apis-common/yup-oauth2"]
default = ["yup-oauth2"]
% endif
