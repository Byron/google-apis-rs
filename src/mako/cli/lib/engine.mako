<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (hub_type, mangle_ident, indent_all_but_first_by, activity_rust_type, setter_fn_name, ADD_PARAM_FN,
                      upload_action_fn)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG, 
                     cmd_ident, call_method_ident, arg_ident, POD_TYPES, flag_ident, ident, JSON_TYPE_RND_MAP)

    v_arg = '<%s>' % VALUE_ARG
    SOPT = 'self.opt.'
    def to_opt_arg_ident(p):
        return SOPT + arg_ident(p.name)

    def borrow_prefix(p):
        ptype = p.get('type', None)
        borrow = ''
        if ptype not in POD_TYPES or ptype in ('string', None):
            borrow = '&'
        return borrow

    STANDARD = 'standard-request'
%>\
<%def name="new(c)">\
<%
    hub_type_name = 'api::' + hub_type(c.schemas, util.canonical_name())
%>\
mod cmn;
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use rustc_serialize::json;

struct Engine {
    opt: Options,
    hub: ${hub_type_name}<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
    fn ${call_method_ident(resource, method)}(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        ${self._method_call_impl(c, resource, method) | indent_all_but_first_by(2)}
    }

    % endfor # each method
% endfor 
    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

## RESOURCE LOOP: check for set primary subcommand
% for resource in sorted(c.rta_map.keys()):
        % if loop.first:
        if \
        % else:
 else if \
        % endif
self.opt.${cmd_ident(resource)} {
        ## METHOD LOOP: Check for method subcommand
        % for method in sorted(c.rta_map[resource]):
            % if loop.first:
            if \
            % else:
 else if \
            % endif
self.opt.${cmd_ident(method)} {
                call_result = self.${call_method_ident(resource, method)}(dry_run, &mut err);
            }\
        % endfor # each method
 else {
                unreachable!();
            }
        }\
% endfor # each resource
 else {
            unreachable!();
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
        }
        (call_result, err_opt)
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: Options) -> Result<Engine, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(&opt.flag_config_dir) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "${util.program_name()}-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "${util.program_name()}",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: ${hub_type_name}::new(hyper::Client::new(), auth),
        };

        match engine._doit(true) {
            (_, Some(err)) => Err(err),
            _ => Ok(engine),
        }
    }

    // Execute the call with all the bells and whistles, informing the caller only if there was an error.
    // The absense of one indicates success.
    fn doit(&self) -> Option<api::Error> {
        self._doit(false).0
    }
}
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
%>\
    ## REQUIRED PARAMETERS
% for p in mc.required_props:
<% 
    prop_name = mangle_ident(p.name)
    prop_type = activity_rust_type(c.schemas, p, allow_optionals=False)
    opt_ident = to_opt_arg_ident(p)
%>\
    % if is_request_value_property(mc, p):
let ${prop_name}: api::${prop_type} = Default::default();
    % elif p.type != 'string':
let ${prop_name}: ${prop_type} = arg_from_str(&${opt_ident}, err, "<${mangle_subcommand(p.name)}>", "${p.type}");
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
            arg_name = to_opt_arg_ident(p)
        call_args.append(borrow + arg_name)
    # end for each required prop
%>\
% if track_download_flag:
let mut download_mode = false;
% endif
let mut call = self.hub.${mangle_ident(resource)}().${mangle_ident(method)}(${', '.join(call_args)});
% if handle_props:
for parg in ${SOPT + arg_ident(VALUE_ARG)}.iter() {
    let (key, value) = parse_kv_arg(&*parg, err);
    match key {
% for p in optional_props:
<% 
    ptype = p.type
    if p.type == 'string' and 'Count' in p.name:
        ptype = 'int64'
    value_unwrap = 'value.unwrap_or("%s")' % JSON_TYPE_RND_MAP[ptype]()
%>\
        "${mangle_subcommand(p.name)}" => {
        % if p.name == 'alt':
            if ${value_unwrap} == "media" {
                download_mode = true;
            }
        % endif
            call = call.${mangle_ident(setter_fn_name(p))}(\
        % if ptype != 'string':
arg_from_str(${value_unwrap}, err, "${mangle_subcommand(p.name)}", "${p.type}")\
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
let protocol = 
% for p in mc.media_params:
    % if loop.first:
    if \
    % else:
    } else if \
    % endif
${SOPT + cmd_ident(p.protocol)} {
        "${p.protocol}"
% endfor # each media param
    } else { 
        unreachable!() 
    };
let mut input_file = input_file_from_opts(&${SOPT + arg_ident(FILE_ARG[1:-1])}, err);
let mime_type = input_mime_from_opts(&${SOPT + arg_ident(MIME_ARG[1:-1])}, err);
% else:
let protocol = "${STANDARD}";
% endif # support upload
if dry_run {
    None
} else {
    assert!(err.issues.len() == 0);
    ## Make the call, handle uploads, handle downloads (also media downloads|json decoding)
    ## TODO: unify error handling
    % if handle_output:
    let mut ostream = writer_from_opts(${SOPT + flag_ident(OUTPUT_FLAG)}, &${SOPT + arg_ident(OUT_ARG[1:-1])});
    % endif # handle output
    match match protocol {
        % if mc.media_params:
        % for p in mc.media_params:
        "${p.protocol}" => call.${upload_action_fn(api.terms.upload_action, p.type.suffix)}(input_file.unwrap(), mime_type.unwrap()),
        % endfor
        % else:
        "${STANDARD}" => call.${api.terms.action}(),
        % endif
        _ => unreachable!(),
    } {
        Err(api_err) => Some(api_err),
        % if mc.response_schema:
        Ok((mut response, output_schema)) => {
        % else:
        Ok(mut response) => {
        % endif # handle output structure
            println!("DEBUG: REMOVE ME {:?}", response);
            ## We are not generating optimal code, but hope it will still be logically correct.
            ## If not, we might build the code in python
            ## TODO: Fix this
            % if track_download_flag:
            if !download_mode {
            % endif
            % if mc.response_schema:
            serde::json::to_writer(&mut ostream, &output_schema).unwrap();
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
            None
        }
    }
}\
</%def>