<%! from util import (estr, enclose_in, hash_comment, library_to_crate_name, to_extern_crate_name) %>\
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
% if documentationLink is not UNDEFINED and documentationLink:
homepage = "${documentationLink}"
% endif
documentation = "${util.doc_base_url()}"
license = "${copyright.license_abbrev}"
keywords = ["${name[:20]}", ${", ".join(estr(cargo.keywords))}]
% if cargo.get('build_script'):
build = "${cargo.build_script}"
% endif

% if cargo.get('is_executable', False):
[[bin]]
name = "${util.program_name()}"
% endif

[dependencies]
hyper = "^ 0.9"
## Must match the one hyper uses, otherwise there are duplicate similarly named `Mime` structs
mime = "^ 0.2.0"
serde = "^ 0.8"
serde_json = "^ 0.8"
yup-oauth2 = { version = "^ 1.0", optional = true, default-features = false }
serde_derive = { version = "^ 0.8", optional = true }
% for dep in cargo.get('dependencies', list()):
${dep}
% endfor

[features]
<%
  api_name = util.library_name()
  crate_name_we_depend_on = None
  
  nightly_features = ["serde_derive", "yup-oauth2/nightly"]
  default_features = ["serde_codegen", "yup-oauth2/with-serde-codegen"]
  
  if make.depends_on_suffix is not None:
    crate_name_we_depend_on = library_to_crate_name(api_name, suffix=make.depends_on_suffix)
    nightly_features.append(crate_name_we_depend_on + '/nightly')
    default_features.append(crate_name_we_depend_on + '/with-serde-codegen')
%>\
default = ["with-serde-codegen"]
nightly = [${','.join(enclose_in('"', nightly_features))}]
with-serde-codegen = [${','.join(enclose_in('"', default_features))}]

[build-dependencies]
serde_codegen = { version = "^ 0.8", optional = true }

% if make.depends_on_suffix is not None:

[dependencies.${crate_name_we_depend_on}]
path = "../${api_name}"
version = "${util.crate_version()}"
optional = true
default-features = false
% endif
