<%
    import os
    from util import (gen_crate_dir, api_index)

    title = 'Google Service Documentation for Rust'

    first_api_prefix = None
    for ti in make.types:
        if ti.documentation_engine != 'rustdoc':
            continue
        for an in sorted(api.list.keys()):
            for v in api.list[an]:
                if api_index(DOC_ROOT, an, v, ti):
                    first_api_prefix = gen_crate_dir(an, v, ti)
                    break
            # for each version
        # for each api name
    # end for each type
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
.text {
  color: #000000;
  font-size: 20px
}
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
        <% 
            has_any_index = False
            types = list()
            for ti in make.types:
                if api_index(DOC_ROOT, an, v, ti):
                    has_any_index = True
                    types.append(ti)
            # end for each type
        %>\
        % if not has_any_index:
            <% continue %>\
        % endif
        <span class="text">${an} ${v} (
        % for ti in types:
            <a class="mod" href="${api_index(DOC_ROOT, an, v, ti)}" title="${ti.id.upper()} docs for the ${an} ${v}">${ti.id.upper()}</a>
            % if not loop.last:
, 
            % endif
        % endfor # each program type
        )</span><br/>
    % endfor # each version
% endfor # each API
</ul>
</body>
</html>