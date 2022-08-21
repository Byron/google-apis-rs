<%! import generator.lib.util as util %>\

## source should be ${self.uri}
## you need to escape the output, using a filter for example
<%def name="gen_info(source)">\
DO NOT EDIT !
This file was generated automatically from '${source}'
DO NOT EDIT !\
</%def>

## This will only work within a substitution, not within python code
<%def name="to_api_version(v)" buffered="True">\
	<% assert len(v) >= 2 and v[0] == 'v'%>\
	## convert it once to int, just to be sure it is an int
${v[1:]}\
</%def>

<%def name="github_source_root_url()" buffered="True">\
${cargo.repo_base_url}/tree/main/${directories.output}/${util.target_directory_name(name, version, make.target_suffix)}\
</%def>

<%def name="library_name()" buffered="True">\
${util.library_name(name, version)}\
</%def>

<%def name="crate_name()" buffered="True">\
${util.library_to_crate_name(util.library_name(name, version), make.target_suffix)}\
</%def>

<%def name="doc_base_url()" buffered="True">\
% if make.id == 'cli':
${cargo.doc_base_url + '/' + util.to_extern_crate_name(self.crate_name())}\
% else:
${util.docs_rs_url(cargo.doc_base_url, self.crate_name(), self.crate_version())}\
% endif
</%def>

<%def name="crate_version()" buffered="True">\
${util.crate_version(cargo.build_version, revision)}\
</%def>

## All crates and standard `use` declaration, required for all examples
## Must be outside of a test function
<%def name="test_prelude()">\
extern crate hyper;
extern crate hyper_rustls;
extern crate ${util.to_extern_crate_name(self.crate_name())} as ${self.library_name()};
</%def>

## Define the canonical name, if present, or name otherwise
<%def name="canonical_name()" buffered="True">\
% if canonicalName is UNDEFINED:
${name}\
% else:
${canonicalName}\
% endif
</%def>

<%def name="program_name()" buffered="True">\
${util.program_name(name, version)}\
</%def>
