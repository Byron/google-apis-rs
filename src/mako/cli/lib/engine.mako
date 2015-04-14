<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from util import hub_type
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG, 
                     cmd_ident, call_method_ident)

    v_arg = '<%s>' % VALUE_ARG
%>\
<%def name="new(c)">\
<%
    hub_type_name = 'api::' + hub_type(c.schemas, util.canonical_name())
%>\
mod cmn;
use cmn::{InvalidOptionsError, JsonTokenStorage};

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};

struct Engine {
    opt: Options,
    config_dir: String,
    hub: ${hub_type_name}<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
% for resource in sorted(c.rta_map.keys()):
    % for method in sorted(c.rta_map[resource]):
    fn ${call_method_ident(resource, method)}(&self, dry_run: bool, err: &mut InvalidOptionsError) -> Option<api::Error> {
        ${self._method_call_impl(c, resource, method)}\
    }

    % endfor # each method
% endfor 
    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error> = None;
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
            config_dir: config_dir,
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

<%def name="_method_call_impl(c, resource, method)">\
None
</%def>