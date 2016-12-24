<%
    import json
    import os
    import yaml
    from util import (api_json_path, library_name, library_to_crate_name,
                      gen_crate_dir, api_index, crates_io_url, program_name,
                      crate_version)

    title = 'Google Service Documentation for Rust'
    # A poor mans merge, just for what we need right now
    def merge_required_fields(map):
      map.cargo.build_version = cargo.build_version
      return map

    # type cache: {'api': type-api.yaml-contents }
    tc = dict()
    for api_type in make.types:
        data = yaml.load_all(open(os.path.join(directories.api_base, 'type-%s.yaml' % api_type)))
        tc[api_type] = merge_required_fields(type(directories)(data.next()))
    # end for each type to load cache for
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
.lib {
  color: #000000;
  font-size: 20px;
  float: left;
  width: 300px;
}
.mod {
  color: #4d76ae;
  font-size: 20px;
}
.mono {
  font-family: monospace;
}
</style>
<script type="text/javascript">
alertShown = false
function onClick(button) {
  selectElementContents(button)
  if (document.execCommand('copy') && !alertShown) {
    msg = "Installation script copied to clipboard.\n"
    
    msg += "\nIt contains no new-lines and will \n"
    msg += "not execute automatically after\n"
    msg += "pasting it into a shell so you can\n"
    msg += "review it beforehand.\n"
    
    msg += "\nThis message will not be shown again."
    alert(msg)
    alertShown = true
  }
}

function selectElementContents(el) {
    if (window.getSelection && document.createRange) {
        var sel = window.getSelection()
        var range = document.createRange()
        range.selectNodeContents(el)
        sel.removeAllRanges()
        sel.addRange(range)
    } else if (document.selection && document.body.createTextRange) {
        var textRange = document.body.createTextRange()
        textRange.moveToElementText(el)
        textRange.select()
    }
}

function onCopy(e) {
  installation_script = '{ command -v rustup 2>&1 >/dev/null || curl https://sh.rustup.rs -sSf | sh } && ' + e.target.textContent
  e.clipboardData.setData('text/plain', installation_script);
  e.preventDefault()
}
</script>
	<title>${title}</title>
</head>
<body>
<H1>${title}</H1>
<ul>
% for an in sorted(api.list.keys()):
    % if an in api.blacklist:
        <% continue %>\
    % endif
    % for v in api.list[an]:
        <% 
            type_names = tc.keys()
            with open(api_json_path(directories.api_base, an, v)) as fp:
              api_data = json.load(fp)
        %>\
        % if api_data is None:
            <% continue %>\
        % endif
        <span class="lib">${an} ${v}</span> 
        % for program_type in type_names:
            <% 
              ad = tc[program_type] 
              revision = api_data.get('revision', None)
            %>\
            <a class="mod" href="${api_index(DOC_ROOT, an, v, ad.make, ad.cargo, revision)}" title="${ad.make.id.upper()} docs for the ${an} ${v}">${ad.make.id.upper()}</a>
            % if program_type == 'api':
            <a href="${crates_io_url(an, v)}/${crate_version(ad.cargo.build_version, revision)}"><img src="${url_info.asset_urls.crates_img}" title="This API on crates.io" height="16" width="16"/></a>
            % else:
            , <button class="mono" onclick="onClick(this)" oncopy="onCopy(event)" title="Copy complete installation script to clipboard">cargo install ${library_to_crate_name(library_name(an, v))}-cli</button>
            % endif
            % if not loop.last:
,           
            % endif
        % endfor # each program type
        <br/>
    % endfor # each version
% endfor # each API
</ul>
</body>
</html>