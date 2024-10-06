<%
    import json
    import os
    import yaml
    from generator.lib.util import (api_json_path, library_name, library_to_crate_name,
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
        data = yaml.load_all(open(os.path.join(directories.api_base, 'type-%s.yaml' % api_type)), Loader = yaml.Loader)
        tc[api_type] = merge_required_fields(type(directories)(next(data)))
    # end for each type to load cache for
%>\
<!DOCTYPE html>
<!--
DO NOT EDIT !
This file was generated automatically by '${self.uri}'
DO NOT EDIT !
-->
<html lang="en">
<head>
  <link rel="stylesheet"
    href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css"
    integrity="sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u"
    crossorigin="anonymous">
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
  <div class="container">
  <h1>${title}</h1>
  <table class="table table-hover">
    <thead>
      <tr>
        <th>API Name</th>
        <th>API Docs</th>
        <th>CLI Docs</th>
        <th>Install</th>
      </tr>
    </thead>
    <tbody>
    % for name in sorted(api.list.keys()):
      % if name in api.blacklist:
        <% continue %>\
      % endif
    % for version in api.list[name]:
      <tr>
        <%
            type_names = ["api", "cli"]
            assert set(type_names) == set(tc.keys()), "The type cache has changed, make sure to update the documentation accordingly"

            with open(api_json_path(directories.api_base, name, version)) as fp:
                metadata = json.load(fp)

            if metadata is None:
                continue

            api_data = tc["api"]
            revision = metadata.get('revision', None)

            api_link = api_index(DOC_ROOT, name, version, api_data['make'],
                api_data['cargo'], revision)

            crates_link = crates_io_url(name, version)
            crates_link += "/"
            crates_link += crate_version(api_data.cargo.build_version, revision)

            cli_data = tc["cli"]
            cli_link = api_index(DOC_ROOT, name, version, cli_data['make'],
                                 cli_data['cargo'], revision)
        %>\
        <td>${name} (${version})</td>
          <td>
            <a href="${api_link}" title="API docs for the ${name} ${version}">API</a>
            <a href="${crates_link}">
              <img src="${url_info.asset_urls.crates_img}"
                title="This API on crates.io" height="16" width="16"/>
            </a>
          </td>
          <td>
            <a href="${cli_link}" title="CLI docs for the ${name} ${version}">
              CLI
            </a>
          </td>
          <td>
            <button class="mono" onclick="onClick(this)"
              oncopy="onCopy(event)"
              title="Copy complete installation script to clipboard">
              cargo install ${library_to_crate_name(library_name(name, version))}-cli
            </button>
          </td>
        </tr>
      % endfor # each version
      % endfor # each API
      </tbody>
    </table>
  </div>
</body>
</html>
