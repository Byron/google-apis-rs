<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from mako.filters import xml_escape
    from util import (hash_comment, new_context, method_default_scope, indent_all_but_first_by, is_repeated_property, custom_sorted)
    from cli import (subcommand_md_filename, new_method_context, SPLIT_START, SPLIT_END, pretty, SCOPE_FLAG,
                     mangle_subcommand, is_request_value_property, FIELD_SEP, PARAM_FLAG, UPLOAD_FLAG, docopt_mode,
                     FILE_ARG, MIME_ARG, OUT_ARG, OUTPUT_FLAG, to_cli_schema, cli_schema_to_yaml, SchemaEntry,
                     STRUCT_FLAG, field_to_value, CTYPE_ARRAY, CTYPE_MAP, to_docopt_arg, FILE_FLAG, MIME_FLAG, 
                     DEFAULT_MIME)

    from copy import deepcopy

    escape_html = lambda n: n.replace('>', r'\>')

    NO_DESC = 'No description provided.'
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
% if 'description' in mc.m:
${mc.m.description | xml_escape}
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
* **${to_docopt_arg(p) | xml_escape}** *(${p.type})*
    - ${p.get('description') or NO_DESC | xml_escape, indent_all_but_first_by(2)}
% if p.get('repeated'):
    - This property can be specified one or more times
% endif
% endfor  # each required property (which is not the request value)
% endif # have required properties
% if mc.request_value:
<%
    request_cli_schema = to_cli_schema(c, mc.request_value)
%>\
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
${cli_schema_to_yaml(request_cli_schema)}
```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

${self._list_schem_args(request_cli_schema)}

${'###'} About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `${FIELD_SEP}` character. Fields can be nested such as in `-${STRUCT_FLAG} f${FIELD_SEP}s${FIELD_SEP}o` .
* The cursor position is set relative to the top-level structure if it starts with `${FIELD_SEP}`, e.g. `-${STRUCT_FLAG} ${FIELD_SEP}s${FIELD_SEP}s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-${STRUCT_FLAG} struct${FIELD_SEP}sub_struct=bar`.
* You can move the cursor one level up by using `${FIELD_SEP}${FIELD_SEP}`. Each additional `${FIELD_SEP}` moves it up one additional level. E.g. `${FIELD_SEP}${FIELD_SEP}${FIELD_SEP}` would go three levels up.

% endif # have request value
% if mc.media_params:
<%
    protocols = [mp.protocol for mp in mc.media_params]
%>\
# Required Upload Flags

This method supports the upload of data, which *requires* all of the following flags to be set:

* **-${UPLOAD_FLAG} ${docopt_mode(protocols)}**
% for mp in mc.media_params:
    - **${mp.protocol}** - ${mp.get('description', NO_DESC).split('\n')[0] | xml_escape}
% endfor # each media param
* **-${FILE_FLAG} ${escape_html(FILE_ARG)}**
    - Path to file to upload. It must be seekable.

The following flag *may* be set: 

* **-${MIME_FLAG} ${escape_html(MIME_ARG)}**
    - the mime type, like '${DEFAULT_MIME}', which is the default

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

You may set the following properties to further configure the call. Please note that `-${PARAM_FLAG}` is followed by one 
or more key-value-pairs, and is called like this `-${PARAM_FLAG} k1=v1 k2=v2` even though the listing below repeats the
`-${PARAM_FLAG}` for completeness.

% for p in custom_sorted(oprops):
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
    - ${p.get('description') or NO_DESC | xml_escape ,indent_all_but_first_by(2)}
</%def>

<%def name="_list_schem_args(schema, cursor_tokens=list(), first_flag=None)">\
<%
    if len(cursor_tokens) == 0:
        cursor_tokens = [FIELD_SEP]

    if first_flag is None:
        first_flag = '-%s ' % STRUCT_FLAG

    def cursor_fmt(cursor):
        fndfi = 0 # first non-dot field index
        for (fndfi, v) in enumerate(cursor):
            if v != FIELD_SEP:
                break
        res = ''.join(cursor[:fndfi]) + FIELD_SEP.join(cursor[fndfi:])
        res += '    '
        return res

    def cursor_arg(field):
        prefix = ''
        if cursor_tokens:
            prefix = cursor_fmt(cursor_tokens)
            del cursor_tokens[:]
        return  prefix + field
%>\
% for fni, fn in enumerate(sorted(schema.fields.keys())):
<% 
    f = schema.fields[fn]
    if fni > 0:
        first_flag = ''
%>\
% if isinstance(f, SchemaEntry):
* `${first_flag}${cursor_arg(mangle_subcommand(fn))}=${field_to_value(f)}`
    - ${f.property.get('description', NO_DESC) | xml_escape, indent_all_but_first_by(2)}
% if f.container_type == CTYPE_ARRAY:
    - Each invocation of this argument appends the given value to the array.
% elif f.container_type == CTYPE_MAP:
    - the value will be associated with the given `key`
% endif # handle container type
% else:
<%
    cursor_tokens.append(mangle_subcommand(fn))
%>\
${self._list_schem_args(f, cursor_tokens, first_flag)}
<%
    assert not cursor_tokens or cursor_tokens[-1] == FIELD_SEP
    if not cursor_tokens:
        cursor_tokens.append(FIELD_SEP)
    cursor_tokens.append(FIELD_SEP) 
%>\
% endif
% endfor
</%def>