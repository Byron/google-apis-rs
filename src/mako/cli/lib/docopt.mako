<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (put_and, supports_scopes)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG)

    v_arg = '<%s>' % VALUE_ARG
    file_arg = '<file>'
    mime_arg = '<mime>'
    out_arg = '<out>'
%>\
<%def name="new(c)">\
<%
    param_used = False
    struct_used = False
    upload_protocols_used = set()
    output_used = False
%>\
docopt!(Args derive Debug, "
Usage: 
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)

    args = list()
    if mc.optional_props or parameters is not UNDEFINED:
        args.append('[-%s %s]...' % (PARAM_FLAG, v_arg))
        param_used = True
    # end paramters

    if mc.request_value:
        args.append('-%s %s...' % (STRUCT_FLAG, v_arg))
        struct_used = True
    # end request_value

    if mc.media_params:
        upload_protocols = [mp.protocol for mp in mc.media_params]
        mode = '|'.join(upload_protocols)
        if len(mc.media_params) > 1:
            mode = '(%s)' % mode
        args.append('-%s %s %s %s' % (UPLOAD_FLAG, mode, file_arg, mime_arg))
        upload_protocols_used = upload_protocols_used|set(upload_protocols)
    # end upload handling

    if mc.response_schema:
        args.append('[-%s %s]' % (OUTPUT_FLAG, out_arg))
        output_used = True
    # handle output
%>\
  ${util.program_name()} [config] ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ${' '.join(args)}
    % endfor # each method
% endfor # end for each resource
  ${util.program_name()} --help

% if param_used|struct_used|output_used or upload_protocols_used:
Options:
% if param_used:
    -${PARAM_FLAG} ${v_arg}  set optional request parameter; ${v_arg} is of form 'name=value'
% endif
% if struct_used:
    -${STRUCT_FLAG} ${v_arg}  set request structure field;
            ${v_arg} supports cursor form 'field[:subfield]...' to 
            set the curor for upcoming values and supports the value form
            'field[:subfield]...=value' to set an actual field.
% endif
% if upload_protocols_used:
    -${UPLOAD_FLAG}  <mode> ${file_arg} ${mime_arg}
            <mode> may be one of the following upload modes: ${put_and(sorted(upload_protocols_used))}
            ${file_arg} path to file to upload. It must be seekable.
            ${mime_arg} the mime type, like 'application/octet-stream', 
            which is the default
% endif
% if output_used:
    -${OUTPUT_FLAG} ${out_arg}
            The `destination` to which to write the server result to. 
            It will either be a json-encoded structure, or the
            media file you are downloading.
            `destination` may be '-' to indicate standard output, or
            a filepath that is to contain the received bytes.
            If unset, it defaults to standard output.
% endif

% endif # any special option is used
Config:
% if supports_scopes(auth):
  --${SCOPE_FLAG} <url>  
            Specify the authentication a method should be executed in. Each scope requires
            the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
% endif scopes
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ${CONFIG_DIR}]
");
</%def>