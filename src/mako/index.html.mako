<%
    import os
    from util import (library_name, library_to_crate_name, to_extern_crate_name)

    def gen_crate_dir(name, version):
        return to_extern_crate_name(library_to_crate_name(library_name(name, version), make.target_suffix))

    def api_index(name, version):
        crate_dir = gen_crate_dir(name, version)
        index_file_path = crate_dir + '/' + crate_dir + '/index.html'
        if os.path.isfile(DOC_ROOT + '/' + index_file_path):
            return index_file_path
        return None

    title = 'Google Rust Client API Docs'

    first_api_prefix = None
    for an in sorted(api.list.keys()):
        for v in api.list[an]:
            if api_index(an, v):
                first_api_prefix = gen_crate_dir(an, v)
                break
        # for each version
    # for each api name
    assert first_api_prefix
%>\
<!DOCTYPE html>
<!--
DO NOT EDIT !
This file was generated automatically by '${self.uri}'
DO NOT EDIT !
-->
<html>
<head>
<link rel="stylesheet" href="${first_api_prefix}/main.css">
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