<%
    import os
    from util import (library_name, library_to_crate_name)

    def api_index(name, version):
        index_file_path = library_to_crate_name(library_name(name, version)) + '/index.html'
        if os.path.isfile(DOC_ROOT + '/' + index_file_path):
            return index_file_path
        return None

    title = 'Google Rust Client API Docs'
%>\
<!DOCTYPE html>
<!--
DO NOT EDIT !
This file was generated automatically by '${self.uri}'
DO NOT EDIT !
-->
<html>
<head>
<link rel="stylesheet" href="main.css">
<style type="text/css">
.mod {
  color: #4d76ae;
  font-size: 20px
}
</style>
	<title>${title}</title>
</head>
<body>
<H1>${title}</H1>
<ul>
% for an in sorted(api.list.keys()):
    % for v in api.list[an]:
    % if not api_index(an, v):
        <% continue %>\
    % endif
    <a class="mod" href="${api_index(an, v)}" title="API docs for ${an} ${v}">${an} ${v}</a><br/>
    % endfor
% endfor
</ul>
</body>
</html>