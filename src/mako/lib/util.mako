<%! import util %>\

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

<%def name="repository_url()" buffered="True">\
${cargo.repo_base_url}/${OUTPUT_DIR}\
</%def>

<%def name="library_name()" buffered="True">\
${util.library_name(name, version)}\
</%def>

<%def name="crate_name()" buffered="True">\
google-${self.library_name()}\
</%def>

## All crates and standard `use` declaration, required for all examples
## Must be outside of a test function
<%def name="test_prelude()">\
extern crate hyper;
extern crate "yup-oauth2" as oauth2;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "${self.crate_name()}" as ${self.library_name()};
</%def>

## Define the canonical name, if present, or name otherwise
<%def name="canonical_name()" buffered="True">\
% if canonicalName is UNDEFINED:
${name}\
% else:
${canonicalName}\
% endif
</%def>