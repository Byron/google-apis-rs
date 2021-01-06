<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (hub_type, mangle_ident, indent_all_but_first_by, activity_rust_type, setter_fn_name, ADD_PARAM_FN,
                      upload_action_fn, is_schema_with_optionals, schema_markers, indent_by, method_default_scope,
                      ADD_SCOPE_FN, TREF, enclose_in)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG,
                     call_method_ident, POD_TYPES, opt_value, ident, JSON_TYPE_VALUE_MAP,
                     KEY_VALUE_ARG, to_cli_schema, SchemaEntry, CTYPE_POD, actual_json_type, CTYPE_MAP, CTYPE_ARRAY,
                     application_secret_path, DEBUG_FLAG, DEBUG_AUTH_FLAG, CONFIG_DIR_FLAG, req_value, MODE_ARG,
                     opt_values, SCOPE_ARG, CONFIG_DIR_ARG, DEFAULT_MIME, field_vec, comma_sep_fields, JSON_TYPE_TO_ENUM_MAP,
                     CTYPE_TO_ENUM_MAP)

    v_arg = '<%s>' % VALUE_ARG
    SOPT = 'self.opt'

    def borrow_prefix(p):
        ptype = p.get('type', None)
        borrow = ''
        if (ptype not in POD_TYPES or ptype is None or p.get('repeated', False)) and ptype is not None:
            borrow = '&'
        return borrow

    def gen_global_parameter_names(parameters):
        if parameters is not UNDEFINED:
            return [pn for pn in sorted(parameters.keys())]
        else:
            return list()
%>\
<%def name="new(c)">\
<%
    hub_type_name = 'api::' + hub_type(c.schemas, util.canonical_name())
%>\
use client::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: ${hub_type_name}<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: ${"Vec<&'static str>"},
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
    fn ${call_method_ident(resource, method)}(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("${CONFIG_DIR_ARG}").unwrap_or("${CONFIG_DIR}")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "${application_secret_path(util.program_name())}",
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
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            ${self._debug_client(DEBUG_FLAG) | indent_all_but_first_by(3)};
<% gpm = gen_global_parameter_names(parameters) %>\
        let engine = Engine {
            opt: opt,
            hub: ${hub_type_name}::new(client, auth),
            gp: ${field_vec(gpm)},
            gpm: vec![
                % for pn in list(pn for pn in gpm if mangle_subcommand(pn) != pn):
                    ("${mangle_subcommand(pn)}", "${pn}"),
                % endfor # each global parameter
                ]
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
            connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
        })
} else {
    hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
}\
</%def>

<%def name="_method_call_impl(c, resource, method)" buffered="True">\
<%
    mc = new_method_context(resource, method, c)
    supports_media_download = mc.m.get('supportsMediaDownload', False)
    handle_output = mc.response_schema or supports_media_download
    optional_props = [p for p in mc.optional_props if not p.get('skip_example', False)]
    optional_prop_names = set(p.name for p in optional_props)

    track_download_flag = (not mc.media_params and
                           supports_media_download and
                          (parameters is not UNDEFINED and 'alt' in parameters) or ('alt' in optional_prop_names))
    handle_props = optional_props or parameters is not UNDEFINED
    if mc.request_value:
        request_cli_schema = to_cli_schema(c, mc.request_value)

    request_prop_type = None
    global_parameter_names = gen_global_parameter_names(parameters)
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
        _ => {
<%
    value_unwrap = 'value.unwrap_or("unset")'
%>\
            let mut found = false;
            for param in &self.gp {
                if key == *param {
                    % if track_download_flag and 'alt' in global_parameter_names:
                    if key == "alt" && ${value_unwrap} == "media" {
                        download_mode = true;
                    }
                    % endif
                    found = true;
                    call = call.${ADD_PARAM_FN}(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, ${value_unwrap});
                    break;
                }
            }
            if !found {
                err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                          {let mut v = Vec::new();
                                                                   v.extend(self.gp.iter().map(|v|*v));
% if comma_sep_fields(optional_prop_names):
                                                                   v.extend([${comma_sep_fields(optional_prop_names)}].iter().map(|v|*v));
% endif
                                                                   v } ));
            }
        }
    }
}
% endif # handle call parameters
% if mc.media_params:
let vals = opt.values_of("${MODE_ARG}").unwrap().collect::<Vec<${'&'}str>>();
let protocol = calltype_from_str(vals[0], [${', '.join('"%s"' % mp.protocol for mp in mc.media_params)}].iter().map(|&v| v.to_string()).collect(), err);
let mut input_file = input_file_from_opts(vals[1], err);
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
            let mut value = json::value::to_value(&output_schema).expect("serde to work");
            remove_json_null_values(&mut value);
            json::to_writer_pretty(&mut ostream, &value).unwrap();
            ostream.flush().unwrap();
            % endif
            % if track_download_flag:
            } else {
            % endif
            % if supports_media_download:
            ## Download is the only option - nothing else matters
            io::copy(&mut response, &mut ostream).unwrap();
            ostream.flush().unwrap();
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

    def flatten_schema_fields(schema, res, fields, cur=list()):
        if len(cur) == 0:
            cur = list()

        opt_access = '.as_mut().unwrap()'
        allow_optionals = allow_optionals_fn(schema)
        if not allow_optionals:
            opt_access = ''
        for fn, f in schema.fields.items():
            cur.append(['%s%s' % (mangle_ident(fn), opt_access), fn])
            fields.add(fn)
            if isinstance(f, SchemaEntry):
                cur[-1][0] = mangle_ident(fn)
                res.append((schema, f, list(cur)))
            else:
                flatten_schema_fields(f, res, fields, cur)
            cur.pop()
        # endfor
    # end utility

    schema_fields = list()
    fields = set()
    flatten_schema_fields(request_cli_schema, schema_fields, fields)
%>\
let mut field_cursor = FieldCursor::default();
let mut object = json::value::Value::Object(Default::default());

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

    ## This type-annotation is not required in nightly (or newer rustc)
    ## TODO(ST): try to remove it once there is a newer stable
    let type_info: Option<(&'static str, JsonTypeInfo)> =
        match &temp_cursor.to_string()[..] {
    % for schema, fe, f in schema_fields:
<%
    pname = FIELD_SEP.join(mangle_subcommand(t[1]) for t in f)
    sname = FIELD_SEP.join(t[1] for t in f)
    ptype = actual_json_type(f[-1][1], fe.actual_property.type)
    jtype = 'JsonType::' + JSON_TYPE_TO_ENUM_MAP[ptype]
    ctype = 'ComplexType::' + CTYPE_TO_ENUM_MAP[fe.container_type]
%>\
            "${pname}" => Some(("${sname}", JsonTypeInfo { jtype: ${jtype}, ctype: ${ctype} })),
            % endfor # each nested field
            _ => {
                let suggestion = FieldCursor::did_you_mean(key, &${field_vec(sorted(fields))});
                err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                None
            }
        };
    if let Some((field_cursor_str, type_info)) = type_info {
        FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
    }
}
let mut ${request_prop_name}: api::${request_prop_type} = json::value::from_value(object).unwrap();
</%def>
