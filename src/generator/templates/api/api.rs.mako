<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="../../lib/util.mako"/>\
<%namespace name="rbuild" file="lib/rbuild.mako"/>\
<%namespace name="mbuild" file="lib/mbuild.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%
    from generator.lib.util import (new_context, hub_type, hub_type_params_s)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()

    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

mod utilities;
pub use utilities::*;

mod hub;
pub use hub::*;

% if c.schemas:
mod schemas;
pub use schemas::*;
% endif

mod method_builders;
pub use method_builders::*;

mod call_builders;
pub use call_builders::*;

pub mod enums;
pub use enums::*;
