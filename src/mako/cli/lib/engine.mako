<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from cli import (mangle_subcommand, new_method_context, PARAM_FLAG, STRUCT_FLAG, UPLOAD_FLAG, OUTPUT_FLAG, VALUE_ARG,
                     CONFIG_DIR, SCOPE_FLAG, is_request_value_property, FIELD_SEP, docopt_mode, FILE_ARG, MIME_ARG, OUT_ARG)

    v_arg = '<%s>' % VALUE_ARG
%>\
<%def name="new(c)">\
mod cmn;
use cmn::{InvalidOptionsError, ArgumentError};

use std::fs;

struct Engine {
    opts: Options,
}


impl Engine {
    fn new(options: Options) -> Result<Engine, InvalidOptionsError> {
        {
            let config_dir = match cmn::assure_config_dir_exists(&options.flag_config_dir) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };
        }

        let mut engine = Engine {
            opts: options,
        };

        Ok(engine)
    }
}
</%def>