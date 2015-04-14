<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import (hub_type, mangle_ident, indent_all_but_first_by, activity_rust_type)
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG, 
                     cmd_ident, call_method_ident, arg_ident, POD_TYPES, flag_ident)

    v_arg = '<%s>' % VALUE_ARG
    SOPT = 'self.opt.'
    def to_opt_arg_ident(p):
        return SOPT + arg_ident(p.name)
%>\
<%def name="new(c)">\
<%
    hub_type_name = 'api::' + hub_type(c.schemas, util.canonical_name())
%>\
mod cmn;
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts};
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
    handle_output = mc.response_schema or mc.m.get('supportsMediaDownload', False)
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
        ptype = p.get('type', None)
        if ptype not in POD_TYPES or ptype in ('string', None):
            borrow = '&'
        arg_name = mangle_ident(p.name)
        if ptype == 'string':
            arg_name = to_opt_arg_ident(p)
        call_args.append(borrow + arg_name)
    # end for each required prop
%>\
let call = self.hub.${mangle_ident(resource)}().${mangle_ident(method)}(${', '.join(call_args)});
## TODO: set parameters
## TODO: parse upload
if dry_run {
    None
} else {
    assert!(err.issues.len() == 0);
    ## Make the call, handle uploads, handle downloads (also media downloads|json decoding)
    ## TODO: unify error handling
    % if mc.media_params:
    return None
    % else:
    % if handle_output:
    let mut ostream = writer_from_opts(${SOPT + flag_ident(OUTPUT_FLAG)}, &${SOPT + arg_ident(OUT_ARG[1:-1])});
    % endif # handle output
    match call.${api.terms.action}() {
        Err(api_err) => Some(api_err),
        % if mc.response_schema:
        Ok((response, output_schema)) => {
        % else:
        Ok(mut response) => {
        % endif # handle output structure
            println!("DEBUG: REMOVE ME {:?}", response);
            % if mc.response_schema:
            serde::json::to_writer(&mut ostream, &output_schema).unwrap();
            % elif mc.m.get('supportsMediaDownload', False):
            ## Download is the only option - nothing else matters
            io::copy(&mut response, &mut ostream).unwrap();
            % endif
            None
        }
    }
    % endif
}\
</%def>