


## This will only work within a substitution, not within python code
<%def name="to_api_version(v)">\
	<% assert len(v) >= 2 and v[0] == 'v'%>\
	## convert it once to int, just to be sure it is an int
${v[1:]}\
</%def>

<%def name="repository_url()">\
${cargo.repo_base_url}/${OUTPUT_DIR}\
</%def>