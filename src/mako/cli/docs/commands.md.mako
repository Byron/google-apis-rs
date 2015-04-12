<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (hash_comment, new_context, method_default_scope, indent_all_but_first_by, is_repeated_property)
    from cli import (subcommand_md_filename, new_method_context, SPLIT_START, SPLIT_END, pretty, SCOPE_FLAG,
                     mangle_subcommand, is_request_value_property, FIELD_SEP, PARAM_FLAG, UPLOAD_FLAG, docopt_mode,
                     FILE_ARG, MIME_ARG, OUT_ARG, OUTPUT_FLAG)

    from copy import deepcopy

    escape_html = lambda n: n.replace('>', r'\>')
%>\
<%
    c = new_context(schemas, resources, context.get('methods'))
%>\
% for resource in sorted(c.rta_map.keys()):
% for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)
%>\
${SPLIT_START} ${subcommand_md_filename(resource, method)}
% if mc.m.description:
${mc.m.description}
% endif # show method description
% if mc.m.get('scopes'):
# Scopes

You will need authorization for \
% if len(mc.m.scopes) > 1:
at least one of the following scopes to make a valid call:

% for s in mc.m.scopes:
* *${s}*
% endfor
% else:
the *${mc.m.scopes[0]}* scope to make a valid call.
% endif # len(scopes) > 1

If unset, the scope for this method defaults to *${method_default_scope(mc.m)}*.
You can set the scope for this method like this: `${util.program_name()} --${SCOPE_FLAG} <scope> ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ...`
% endif # have method scopes
<%
    rprops = [p for p in mc.required_props if not is_request_value_property(mc, p)]
    oprops = [p for p in mc.optional_props if not p.get('skip_example', False)]

    smd = mc.m.get('supportsMediaDownload', False)
%>\
% if rprops:
# Required Scalar ${len(rprops) > 1 and 'Arguments' or 'Argument'}
% for p in rprops:
* **<${mangle_subcommand(p.name)}\>**
    - ${p.get('description') or 'No description provided' | indent_all_but_first_by(2)}
% endfor  # each required property (which is not the request value)
% endif # have required properties
% if mc.request_value:
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
"scalar_int": 5,
"struct": {
    "scalar_float": 2.4
    "sub_struct": {
        "strings": ["baz", "bar"],
        "mapping": HashMap,
    }
}
"scalar_str": "foo",
```

can be set completely with the following arguments. Note how the cursor position is adjusted the respective fields:
```
-r scalar_int=2 -r struct -r scalar_float=4.2 -r sub_struct -r strings=first -r strings=second -r mapping=key=value -r ${FIELD_SEP} -r scalar_str=other
```

* The cursor position is always set relative to the current one, unless the field name starts with the '${FIELD_SEP}' character. Fields can be nested such as in `-r f${FIELD_SEP}s${FIELD_SEP}o` .
* **Lists** are always appended to, in the example, the list at `struct${FIELD_SEP}sub_struct${FIELD_SEP}strings` will have the value `["first", "second"]`.
* **Mappings** are set using the `key=value` form.
* You can also set nested fields without setting the cursor explicitly. For example, to set the mapping from the root, you would specify `-r struct${FIELD_SEP}sub_struct${FIELD_SEP}mapping=foo=bar`. In case the cursor is not at the root, you may explicitly drill down from the root using a leading '${FIELD_SEP}' character.

% endif # have request value
% if mc.media_params:
<%
    protocols = [mp.protocol for mp in mc.media_params]
%>\
# Required Upload Flags

This method supports the upload of data, using the following protocol${len(mc.media_params) > 1 and 's' or ''}:

* **-${UPLOAD_FLAG} ${docopt_mode(protocols)} ${escape_html(FILE_ARG)} ${escape_html(MIME_ARG)}**
% for mp in mc.media_params:
    - **${mp.protocol}** - ${mp.description.split('\n')[0]}
% endfor # each media param
    - **${escape_html(FILE_ARG)}**
        + Path to file to upload. It must be seekable.
    - **${escape_html(MIME_ARG)}**
        + the mime type, like 'application/octet-stream', which is the default
% endif # have upload capabilities
% if mc.response_schema or smd:
# Optional Output Flags

% if mc.response_schema:
The method's return value a JSON encoded structure, which will be written to standard output by default.
% endif
% if smd:

% if mc.response_schema:
As this method supports **media download**, you may specify the `-${PARAM_FLAG} alt=media` flag to set the output to be an octet stream of the underlying media. In that case, you will not receive JSON output anymore.
% else:
The method's return value is a byte stream of the downloadable resource.
% endif # handle response schema
% endif # support media download

* **-${OUTPUT_FLAG} ${escape_html(OUT_ARG)}**
    - *${escape_html(OUT_ARG)}* specifies the *destination* to which to write the server's result to.
% if smd and mc.response_schema:
      It will either be a JSON-encoded structure, or the media file you are downloading.
% elif smd:
      It will be a byte stream of the downloadable resource.
% else:
      It will be a JSON-encoded structure.
% endif
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
% endif # have output
% if oprops:
# Optional Method Properties

You may set the following properties to further configure the call.

% for p in sorted(oprops):
${self._md_property(p)}
% endfor 
% endif # optional method properties
% if parameters is not UNDEFINED:
# Optional General Properties

The following properties can configure any call, and are not specific to this method.

% for pn in sorted(parameters.keys()):
<% 
    p = deepcopy(parameters[pn])
    p.name = pn
%>\
${self._md_property(p)}
% endfor 
% endif # general parameters
${SPLIT_END}
% endfor # each method
% endfor # each resource

<%def name="_md_property(p)">\
* **-${PARAM_FLAG} ${mangle_subcommand(p.name)}=${p.type}**
    - ${p.get('description') or "No description provided" | indent_all_but_first_by(2)}
</%def>