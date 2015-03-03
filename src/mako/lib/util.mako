<%! import util %>\

## source should be ${self.uri}
## you need to escape the output, using a filter for example
<%def name="gen_info(source)">\
DO NOT EDIT !
This file was generated automatically from '${source}'
DO NOT EDIT !\
</%def>

## This will only work within a substitution, not within python code
<%def name="to_api_version(v)">\
	<% assert len(v) >= 2 and v[0] == 'v'%>\
	## convert it once to int, just to be sure it is an int
${v[1:]}\
</%def>

<%def name="repository_url()">\
${cargo.repo_base_url}/${OUTPUT_DIR}\
</%def>

<%def name="library_name()">\
${util.library_name(name, version)}\
</%def>