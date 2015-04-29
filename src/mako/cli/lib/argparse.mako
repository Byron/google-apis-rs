<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (put_and, supports_scopes, api_index, indent_by, enclose_in)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG, 
                     CONFIG_DIR_FLAG, KEY_VALUE_ARG, to_docopt_arg, DEBUG_FLAG, DEBUG_AUTH_FLAG)

    def rust_boolean(v):
        return v and 'true' or 'false'

    def rust_optional(v):
        if v is None:
            return 'None'
        if isinstance(v, bool):
            v = v and 'true' or 'false'
        elif isinstance(v, basestring):
            v = '"%s"' % v
        elif isinstance(v, list):
            v = 'vec![%s]' % ','.join('"%s"' % p for p in v)
        return 'Some(%s)' % v
%>\
<%def name="grammar(c)">\
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)

    args = list()
    for p in mc.required_props:
        if is_request_value_property(mc, p):
            continue
        args.append(to_docopt_arg(p))
    # end for each required property

    if mc.request_value:
        args.append('-%s %s...' % (STRUCT_FLAG, '<%s>' % KEY_VALUE_ARG))
    # end request_value

    if mc.media_params:
        upload_protocols = [mp.protocol for mp in mc.media_params]
        mode = docopt_mode(upload_protocols)
        args.append('-%s %s <%s> <%s>' % (UPLOAD_FLAG, mode, FILE_ARG, MIME_ARG))
    # end upload handling

    if mc.optional_props or parameters is not UNDEFINED:
        args.append('[-%s %s...]' % (PARAM_FLAG, '<%s>' % VALUE_ARG))
    # end paramters
    
    if mc.response_schema or mc.m.get('supportsMediaDownload', False):
        args.append('[-%s <%s>]' % (OUTPUT_FLAG, OUT_ARG))
    # handle output
%>\
  ${util.program_name()} [options] ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ${' '.join(args)}
    % endfor # each method
% endfor # end for each resource
  ${util.program_name()} --help

All documentation details can be found at
${cargo.doc_base_url + '/' + api_index(cargo.doc_base_url, name, version, make, check_exists=False)}

Configuration:
% if supports_scopes(auth):
  --${SCOPE_FLAG} <url>  
            Specify the authentication a method should be executed in. Each scope 
            requires the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
% endif scopes
  --${CONFIG_DIR_FLAG} <folder>
            A directory into which we will store our persistent data. Defaults to 
            a user-writable directory that we will create during the first invocation.
            [default: ${CONFIG_DIR}]
  --${DEBUG_FLAG}
            Output all server communication to standard error. `tx` and `rx` are placed 
            into the same stream.
  --${DEBUG_AUTH_FLAG}
            Output all communication related to authentication to standard error. `tx` 
            and `rx` are placed into the same stream.
</%def>



<%def name="new(c)" buffered="True">\
<%
    url_info = "All documentation details can be found at" + \
                cargo.doc_base_url + '/' + api_index(cargo.doc_base_url, name, version, make, check_exists=False)

    # list of tuples
    # (0) = long name
    # (1) = description
    # (2) = argument name, no argument if no argument
    global_args = list()
    if supports_scopes(auth):
        global_args.append((
            SCOPE_FLAG,
            "Specify the authentication a method should be executed in. Each scope "
            "requires the user to grant this application permission to use it."
            "If unset, it defaults to the shortest scope url for a particular method.",
            'url'
        ))
    # end add scope arg
    global_args.append((
        CONFIG_DIR_FLAG,
        "A directory into which we will store our persistent data. Defaults to "
        "a user-writable directory that we will create during the first invocation."
        "[default: ${CONFIG_DIR}]",
        'folder',
    ))

    global_args.append((
        DEBUG_FLAG,  
        "Output all server communication to standard error. `tx` and `rx` are placed "
        "into the same stream.",
        None
    ))

    global_args.append((
        DEBUG_AUTH_FLAG,
        "Output all communication related to authentication to standard error. `tx` "
        "and `rx` are placed into the same stream.",
        None
    ))
%>\
use cmn::UploadProtocol;

let mut app = App::new("${util.program_name()}")
<%block filter="indent_by(7)">\
.author("${', '.join(cargo.authors)}")
.version("${cargo.build_version}")
% if description is not UNDEFINED:
.about("${description}")
% endif
.after_help("${url_info}")
% for flag, desc, arg_name in global_args:
.arg(Arg::with_name("${arg_name or flag}")
        .long("${flag}")
        .help("${desc}")
        .takes_value(${rust_boolean(arg_name)}))\
% if loop.last:
;
% else:

% endif
% endfor
let arg_data = [
% for resource in sorted(c.rta_map.keys()):
<%block filter="indent_by(4)">\
("${mangle_subcommand(resource)}", vec![
    % for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)

    # A list of tuples
    # (0) = short flag, like -c
    # (1) = param description or None
    # (2) = argument name, or None if there is no argument
    # (3) = is required (bool)
    # (4) = allow multi-use
    args = list()
    for p in mc.required_props:
        if is_request_value_property(mc, p):
            continue
        args.append((
            None,
            p.get('description'),
            mangle_subcommand(p.name),
            True,
            False,
            None,
        ))
    # end for each required property

    if mc.request_value:
        args.append((
                STRUCT_FLAG,
                "Set various fields of the request structure",
                KEY_VALUE_ARG,
                True,
                True,
                None
            ))
    # end request_value

    if mc.media_params:
        upload_protocols = [mp.protocol for mp in mc.media_params]
        args.append((
                UPLOAD_FLAG,
                "Specify which file to upload",
                "mode",
                False,
                False,
                upload_protocols
            ))
        ## args.append('-%s %s %s %s' % (UPLOAD_FLAG, mode, FILE_ARG, MIME_ARG))
    # end upload handling

    if mc.optional_props or parameters is not UNDEFINED:
        args.append((
                PARAM_FLAG,
                "Set various fields of the request structure",
                VALUE_ARG,
                False,
                True,
                None
            ))
    # end paramters
    
    if mc.response_schema or mc.m.get('supportsMediaDownload', False):
        args.append((
                OUTPUT_FLAG,
                "Specify the file into which to write the programs output",
                OUT_ARG,
                False,
                False,
                None
            ))
    # handle output
%>\
    ("${mangle_subcommand(method)}",  ${rust_optional(mc.m.get('description'))}, 
          vec![
            % for flag, desc, arg_name, required, multi, upload_protocols in args:
            (${rust_optional(arg_name)},
             ${rust_optional(flag)},
             ${rust_optional(desc)},
             ${rust_optional(required)},
             ${rust_optional(multi)},
             ${rust_optional(upload_protocols)}),
            % if not loop.last:

            % endif
            % endfor
          ]),
    % endfor # each method
    ]),
</%block>
% endfor # end for each resource
];

for &(main_command_name, ref subcommands) in &arg_data {
    let mut mcmd = SubCommand::new(main_command_name);
    for &(sub_command_name, ref desc, ref args) in subcommands {
        let mut scmd = SubCommand::new(sub_command_name);
        if let &Some(desc) = desc {
            scmd = scmd.about(desc);
        }
        for &(ref arg_name, ref flag, ref desc, ref required, ref multi, ref protocols) in args {
            let mut arg = Arg::with_name(match (arg_name, flag) {
                                                (&Some(an), _) => an,
                                                (_, &Some(f)) => f,
                                                _ => unreachable!(),
                                            });
            if let &Some(short_flag) = flag {
                arg = arg.short(short_flag);
            }
            if let &Some(desc) = desc {
                arg = arg.help(desc);
            }
            if arg_name.is_some() && flag.is_some() {
                arg = arg.takes_value(true);
            }
            if let &Some(required) = required {
                arg = arg.required(required);
            }
            if let &Some(multi) = multi {
                arg = arg.multiple(multi);
            }
            if let &Some(ref protocols) = protocols {
                arg = arg.possible_values(protocols.clone());
                arg = arg.requires("file");
                arg = arg.requires("mime");

                scmd = scmd.arg(Arg::with_name("file")
                                    .short("f")
                                    .required(false)
                                    .help("The file to upload")
                                    .takes_value(true));
                scmd = scmd.arg(Arg::with_name("mime")
                                    .short("m")
                                    .required(false)
                                    .help("The file's mime time, like 'application/octet-stream'")
                                    .takes_value(true));
            }
            scmd = scmd.arg(arg);
        }
        mcmd = mcmd.subcommand(scmd);
    }
    app = app.subcommand(mcmd);
}
let matches = app.get_matches();
</%block>
</%def>