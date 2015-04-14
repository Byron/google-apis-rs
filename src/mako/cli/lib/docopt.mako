<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (put_and, supports_scopes)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG, 
                     CONFIG_DIR_FLAG)

    v_arg = '<%s>' % VALUE_ARG
%>\
<%def name="new(c)">\
<%
    param_used = False
    struct_used = False
    upload_protocols_used = set()
    output_used = False
%>\
docopt!(Options derive Debug, "
Usage: 
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)

    args = list()
    for p in mc.required_props:
        if is_request_value_property(mc, p):
            continue
        args.append('<%s>' % mangle_subcommand(p.name))
    # end for each required property

    if mc.request_value:
        args.append('-%s %s...' % (STRUCT_FLAG, v_arg))
        struct_used = True
    # end request_value

    if mc.media_params:
        upload_protocols = [mp.protocol for mp in mc.media_params]
        mode = docopt_mode(upload_protocols)
        args.append('-%s %s %s %s' % (UPLOAD_FLAG, mode, FILE_ARG, MIME_ARG))
        upload_protocols_used = upload_protocols_used|set(upload_protocols)
    # end upload handling

    if mc.optional_props or parameters is not UNDEFINED:
        args.append('[-%s %s]...' % (PARAM_FLAG, v_arg))
        param_used = True
    # end paramters
    
    if mc.response_schema or mc.m.get('supportsMediaDownload', False):
        args.append('[-%s %s]' % (OUTPUT_FLAG, OUT_ARG))
        output_used = True
    # handle output
%>\
  ${util.program_name()} [options] ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ${' '.join(args)}
    % endfor # each method
% endfor # end for each resource
  ${util.program_name()} --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
% if supports_scopes(auth):
  --${SCOPE_FLAG} <url>  
            Specify the authentication a method should be executed in. Each scope requires
            the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
% endif scopes
  --${CONFIG_DIR_FLAG} <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ${CONFIG_DIR}]
");
</%def>