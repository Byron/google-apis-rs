<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG)

    v_arg = '<%s>' % VALUE_ARG
%>\
<%def name="new(c)">\
mod cmn;
use cmn::InvalidOptionsError;

use oauth2::ApplicationSecret;

struct Engine {
    opts: Options,
    config_dir: String,
    secret: ApplicationSecret,
}


impl Engine {
    fn new(options: Options) -> Result<Engine, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(&options.flag_config_dir) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "${util.program_name()}-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let mut engine = Engine {
            opts: options,
            config_dir: config_dir,
            secret: secret,
        };

        Ok(engine)
    }
}
</%def>