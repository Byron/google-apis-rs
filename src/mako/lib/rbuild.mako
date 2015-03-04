<%!
	from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, singular, hub_type)
%>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a Resource builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, c)">\
<% hub_type_name = hub_type(canonicalName) %>\
/// A builder providing access to all methods supported on *${singular(resource)}* resources.
/// It is not used directly, but through the `${hub_type_name}` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
<%block filter="rust_doc_test_norun, rust_doc_comment">\
${util.test_prelude()}\

<%block filter="rust_test_fn_invisible">\
${lib.test_hub(hub_type_name, comments=False)}\

// Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
// like ${put_and(sorted('`%s(...)`' % f for f in c.rta_map[resource]))}
// to build up your call.
let rb = hub.${resource}();
</%block>
</%block>
pub struct ${rb_type(resource)}<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a ${hub_type_name}<C, NC, A>
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ${rb_type(resource)}<'a, C, NC, A> {}
</%def>