<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (hub_type, mangle_ident, indent_all_but_first_by, activity_rust_type, setter_fn_name, ADD_PARAM_FN,
                      upload_action_fn, is_schema_with_optionals, schema_markers, indent_by, method_default_scope,
                      ADD_SCOPE_FN, TREF)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG, 
                     call_method_ident, POD_TYPES, opt_value, ident, JSON_TYPE_VALUE_MAP,
                     KEY_VALUE_ARG, to_cli_schema, SchemaEntry, CTYPE_POD, actual_json_type, CTYPE_MAP, CTYPE_ARRAY,
                     application_secret_path, DEBUG_FLAG, DEBUG_AUTH_FLAG, CONFIG_DIR_FLAG, req_value, MODE_ARG, 
                     opt_values, SCOPE_ARG, CONFIG_DIR_ARG, DEFAULT_MIME)

    v_arg = '<%s>' % VALUE_ARG
    SOPT = 'self.opt'

    def borrow_prefix(p):
        ptype = p.get('type', None)
        borrow = ''
        if (ptype not in POD_TYPES or ptype is None or p.get('repeated', False)) and ptype is not None:
            borrow = '&'
        return borrow
%>\
<%def name="new(c)">\
<%
    hub_type_name = 'api::' + hub_type(c.schemas, util.canonical_name())
%>\
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde::json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: ${hub_type_name}<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl<'n, 'a> Engine<'n, 'a> {
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
    fn ${call_method_ident(resource, method)}(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        ${self._method_call_impl(c, resource, method) | indent_all_but_first_by(2)}
    }

    % endfor # each method
% endfor 
    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
## RESOURCE LOOP: check for set primary subcommand
        match ${SOPT + '.subcommand()'} {
% for resource in sorted(c.rta_map.keys()):
            ("${mangle_subcommand(resource)}", Some(opt)) => {
                match opt.subcommand() {
                    % for method in sorted(c.rta_map[resource]):
                    ("${mangle_subcommand(method)}", Some(opt)) => {
                        call_result = self.${call_method_ident(resource, method)}(opt, dry_run, &mut err);
                    },
                    % endfor # each method
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("${mangle_subcommand(resource)}".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
% endfor # each resource
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", ${SOPT}.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("${CONFIG_DIR_ARG}").unwrap_or("${CONFIG_DIR}")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "${application_secret_path(util.program_name())}", 
                                                         "${api.credentials.replace('"', r'\"')}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        ${self._debug_client(DEBUG_AUTH_FLAG) | indent_all_but_first_by(10)},
                                        JsonTokenStorage {
                                          program_name: "${util.program_name()}",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            ${self._debug_client(DEBUG_FLAG) | indent_all_but_first_by(3)};
        let engine = Engine {
            opt: opt,
            hub: ${hub_type_name}::new(client, auth),
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}
</%def>

<%def name="_debug_client(flag_name)" buffered="True">\
if opt.is_present("${flag_name}") {
    hyper::Client::with_connector(mock::TeeConnector {
            connector: hyper::net::HttpConnector(None) 
        })
} else {
    hyper::Client::new()
}\
</%def>

<%def name="_method_call_impl(c, resource, method)" buffered="True">\
<%
    mc = new_method_context(resource, method, c)
    supports_media_download = mc.m.get('supportsMediaDownload', False)
    handle_output = mc.response_schema or supports_media_download
    optional_props = [p for p in mc.optional_props if not p.get('skip_example', False)]
    optional_prop_names = set(p.name for p in optional_props)

    global_parameter_names = set()
    track_download_flag = (not mc.media_params and
                           supports_media_download and 
                          (parameters is not UNDEFINED and 'alt' in parameters) or ('alt' in optional_prop_names))
    if parameters is not UNDEFINED:
        global_parameter_names = list(pn for pn in sorted(parameters.keys()) if pn not in optional_prop_names)
    handle_props = optional_props or parameters is not UNDEFINED
    if mc.request_value:
        request_cli_schema = to_cli_schema(c, mc.request_value)

    request_prop_type = None
%>\
    ## REQUIRED PARAMETERS
% for p in mc.required_props:
<% 
    prop_name = mangle_ident(p.name)
    prop_type = activity_rust_type(c.schemas, p, allow_optionals=False)
%>\
    % if is_request_value_property(mc, p):
<% request_prop_type = prop_type %>\
${self._request_value_impl(c, request_cli_schema, prop_name, request_prop_type)}\
    % elif p.type != 'string':
    % if p.get('repeated', False): 
let ${prop_name}: Vec<${prop_type} = Vec::new();
for (arg_id, arg) in ${opt_values(mangle_subcommand(p.name))}.enumerate() {
    ${prop_name}.push(arg_from_str(&arg, err, "<${mangle_subcommand(p.name)}>", arg_id), "${p.type}"));
}
    % else:
let ${prop_name}: ${prop_type} = arg_from_str(&${opt_value(p.name)}, err, "<${mangle_subcommand(p.name)}>", "${p.type}");
    % endif # handle repeated values
    % endif # handle request value
% endfor # each required parameter
<%
    call_args = list()
    for p in mc.required_props:
        borrow = ''
        # if type is not available, we know it's the request value, which should also be borrowed
        borrow = borrow_prefix(p)
        arg_name = mangle_ident(p.name)
        if p.get('type', '') == 'string':
            if p.get('repeated', False):
                arg_name = opt_values(p.name) + '.map(|&v| v.to_string()).collect::<Vec<String>>()'
            else:
                arg_name = opt_value(p.name)
        call_args.append(borrow + arg_name)
    # end for each required prop
%>\
% if track_download_flag:
let mut download_mode = false;
% endif
let mut call = self.hub.${mangle_ident(resource)}().${mangle_ident(method)}(${', '.join(call_args)});
% if handle_props:
for parg in ${opt_values(VALUE_ARG)} {
    let (key, value) = parse_kv_arg(&*parg, err, false);
    match key {
% for p in optional_props:
<% 
    ptype = actual_json_type(p.name, p.type)
    value_unwrap = 'value.unwrap_or("%s")' % JSON_TYPE_VALUE_MAP[ptype]
%>\
        "${mangle_subcommand(p.name)}" => {
        % if p.name == 'alt':
            if ${value_unwrap} == "media" {
                download_mode = true;
            }
        % endif
            call = call.${mangle_ident(setter_fn_name(p))}(\
        % if ptype != 'string':
arg_from_str(${value_unwrap}, err, "${mangle_subcommand(p.name)}", "${ptype}")\
        % else:
${value_unwrap}\
        % endif # handle conversion
);
        },
% endfor # each property
    % if parameters is not UNDEFINED:
    % for pn in global_parameter_names:
        \
    % if not loop.first:
|\
    % endif
"${mangle_subcommand(pn)}"\
    % if not loop.last:

    % endif
    % endfor # each global parameter
 => {
<%
    value_unwrap = 'value.unwrap_or("unset")'
%>\
    % if track_download_flag and 'alt' in global_parameter_names:
            if key == "alt" && ${value_unwrap} == "media" {
                download_mode = true;
            }
    % endif
            let map = [
            % for pn in list(pn for pn in global_parameter_names if mangle_subcommand(pn) != pn):
                ("${mangle_subcommand(pn)}", "${pn}"),
            % endfor # each global parameter
            ];
            call = call.${ADD_PARAM_FN}(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, ${value_unwrap})
        },
    % endif # handle global parameters
        _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
    }
}
% endif # handle call parameters
% if mc.media_params:
let protocol = CallType::Upload(UploadProtocol::from(${req_value(MODE_ARG)}));
let mut input_file = input_file_from_opts(${req_value(FILE_ARG)}, err);
let mime_type = input_mime_from_opts(${opt_value(MIME_ARG, default=DEFAULT_MIME)}, err);
% else:
let protocol = CallType::Standard;
% endif # support upload
if dry_run {
    Ok(())
} else {
    assert!(err.issues.len() == 0);
    % if method_default_scope(mc.m):
    for scope in ${opt_values(SCOPE_ARG, opt=SOPT)} {
        call = call.${ADD_SCOPE_FN}(scope);
    }
    % endif
    ## Make the call, handle uploads, handle downloads (also media downloads|json decoding)
    % if handle_output:
    let mut ostream = match writer_from_opts(opt.value_of("${(OUT_ARG)}")) {
        Ok(mut f) => f,
        Err(io_err) => return Err(DoitError::IoError(${opt_value(OUT_ARG, default='-')}.to_string(), io_err)),
    };
    % endif # handle output
    match match protocol {
        % if mc.media_params:
        % for p in mc.media_params:
        CallType::Upload(UploadProtocol::${p.protocol.capitalize()}) => call.${upload_action_fn(api.terms.upload_action, p.type.suffix)}(input_file.unwrap(), mime_type.unwrap()),
        % endfor
        CallType::Standard => unreachable!()
        % else:
        CallType::Standard => call.${api.terms.action}(),
        _ => unreachable!()
        % endif
    } {
        Err(api_err) => Err(DoitError::ApiError(api_err)),
        % if mc.response_schema:
        Ok((mut response, output_schema)) => {
        % else:
        Ok(mut response) => {
        % endif # handle output structure
            ## We are not generating optimal code, but hope it will still be logically correct.
            ## If not, we might build the code in python
            ## TODO: Fix this
            % if track_download_flag:
            if !download_mode {
            % endif
            % if mc.response_schema:
            serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
            % endif
            % if track_download_flag:
            } else {
            % endif
            % if supports_media_download:
            ## Download is the only option - nothing else matters
            io::copy(&mut response, &mut ostream).unwrap();
            % endif
            % if track_download_flag:
            }
            % endif
            Ok(())
        }
    }
}\
</%def>

<%def name="_request_value_impl(c, request_cli_schema, request_prop_name, request_prop_type)">
<%
    allow_optionals_fn = lambda s: is_schema_with_optionals(schema_markers(s, c, transitive=False))

    def flatten_schema_fields(schema, res, init_fn_map, cur=list(), init_call=None):
        if len(cur) == 0:
            init_call = ''
            cur = list()

        opt_access = '.as_mut().unwrap()'
        allow_optionals = allow_optionals_fn(schema)
        parent_init_call = init_call
        if not allow_optionals:
            opt_access = ''
        for fn, f in schema.fields.iteritems():
            cur.append(['%s%s' % (mangle_ident(fn), opt_access), fn])
            if isinstance(f, SchemaEntry):
                cur[-1][0] = mangle_ident(fn)
                res.append((init_call, schema, f, list(cur)))
            else:
                if allow_optionals:
                    init_fn_name = 'request_%s_init' % '_'.join(mangle_ident(t[1]) for t in cur)
                    init_call = init_fn_name + '(&mut request);'
                    struct_field = 'request.' + '.'.join('%s%s' % (mangle_ident(t[1]), opt_access) for t in cur[:-1])
                    if len(cur) > 1:
                        struct_field += '.'
                    struct_field += mangle_ident(cur[-1][1])
                    init_fn =  "fn %s(request: &mut api::%s) {\n" % (init_fn_name, request_prop_type)
                    if parent_init_call:
                        pcall = parent_init_call[:parent_init_call.index('(')] + "(request)"
                        init_fn += "    %s;\n" % pcall
                    init_fn += "    if %s.is_none() {\n" % struct_field
                    init_fn += "        %s = Some(Default::default());\n" % struct_field
                    init_fn += "    }\n"
                    init_fn += "}\n"
                               
                    init_fn_map[init_fn_name] = init_fn
                # end handle init
                flatten_schema_fields(f, res, init_fn_map, cur, init_call)
            cur.pop()
        # endfor
    # end utility

    schema_fields = list()
    init_fn_map = dict()
    flatten_schema_fields(request_cli_schema, schema_fields, init_fn_map)
%>\
let mut ${request_prop_name} = api::${request_prop_type}::default();
let mut field_cursor = FieldCursor::default();
for kvarg in ${opt_values(KEY_VALUE_ARG)} {
    let last_errc = err.issues.len();
    let (key, value) = parse_kv_arg(&*kvarg, err, false);
    let mut temp_cursor = field_cursor.clone();
    if let Err(field_err) = temp_cursor.set(&*key) {
        err.issues.push(field_err);
    }
    if value.is_none() {
        field_cursor = temp_cursor.clone();
        if err.issues.len() > last_errc {
            err.issues.remove(last_errc);
        }
        continue;
    }
    % for name in sorted(init_fn_map.keys()):
${init_fn_map[name] | indent_by(4)}
    % endfor
    match &temp_cursor.to_string()[..] {
    % for init_call, schema, fe, f in schema_fields:
<%
    ptype = actual_json_type(f[-1][1], fe.actual_property.type)
    value_unwrap = 'value.unwrap_or("%s")' % JSON_TYPE_VALUE_MAP[ptype]
    pname = FIELD_SEP.join(mangle_subcommand(t[1]) for t in f)

    allow_optionals = True
    opt_prefix = 'Some('
    opt_suffix = ')'
    struct_field = 'request.' + '.'.join(t[0] for t in f)

    opt_init = 'if ' + struct_field + '.is_none() {\n'
    opt_init += '   ' + struct_field + ' = Some(Default::default());\n'
    opt_init += '}\n'

    opt_access = '.as_mut().unwrap()'
    if not allow_optionals_fn(schema):
        opt_prefix = opt_suffix = opt_access = opt_init = ''
%>\
        "${pname}" => {
            % if init_call:
                ${init_call}
            % endif
        % if fe.container_type == CTYPE_POD:
                ${struct_field} = ${opt_prefix}\
        % elif fe.container_type == CTYPE_ARRAY:
                ${opt_init | indent_all_but_first_by(4)}\
                ${struct_field}${opt_access}.push(\
        % elif fe.container_type == CTYPE_MAP:
                ${opt_init | indent_all_but_first_by(4)}\
let (key, value) = parse_kv_arg(${value_unwrap}, err, true);
                ${struct_field}${opt_access}.insert(key.to_string(), \
        % endif # container type handling
        % if ptype != 'string':
arg_from_str(${value_unwrap}, err, "${pname}", "${ptype}")\
        % else:
${value_unwrap}.to_string()\
        % endif
        % if fe.container_type == CTYPE_POD:
${opt_suffix}\
        % else:
)\
        % endif
;
            },
        % endfor # each nested field
        _ => {
            err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
        }
    }
}
</%def>