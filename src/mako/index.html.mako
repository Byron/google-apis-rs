<%
    import os
    import yaml
    from util import (gen_crate_dir, api_index, crates_io_url)

    title = 'Google Service Documentation for Rust'

    # type cache: {'api': type-api.yaml-contents }
    tc = dict()
    for api_type in make.types:
        data = yaml.load_all(open(os.path.join(directories.api_base, 'type-%s.yaml' % api_type)))
        tc[api_type] = type(directories)(data.next())
    # end for each type to load cache for

    first_api_prefix = None
    for ad in tc.values():
        if ad.make.documentation_engine != 'rustdoc':
            continue
        for an in sorted(api.list.keys()):
            for v in api.list[an]:
                if api_index(DOC_ROOT, an, v, ad.make):
                    first_api_prefix = gen_crate_dir(an, v, ad.make)
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
            type_names = list()
            for api_name, ad in tc.iteritems():
                if api_index(DOC_ROOT, an, v, ad.make):
                    has_any_index = True
                    type_names.append(api_name)
            # end for each type
        %>\
        % if not has_any_index:
            <% continue %>\
        % endif
        <span class="text">${an} ${v} (
        % for api_name in type_names:
            <% ad = tc[api_name] %>
            <a class="mod" href="${api_index(DOC_ROOT, an, v, ad.make)}" title="${ad.make.id.upper()} docs for the ${an} ${v}">${ad.make.id.upper()}</a>
            % if api_name == 'api':
            <a href="${crates_io_url(an, v)}"><img src="${html_index.asset_urls.crates_img}" title="This API on crates.io" height="16" width="16"/></a>
            % endif
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