<%!
	from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, mb_type, singular, hub_type)
%>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a Call builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, method, c)">\
<% hub_type_name = hub_type(canonicalName) %>\
/// A builder for the *${method}* method supported by a *${singular(resource)}* resource.
/// It is not used directly, but through a `${rb_type(resource)}`.
///
/// # Example
///
/// Instantiate a resource method builder
///
<%block filter="rust_doc_test_norun, rust_doc_comment">\
${util.test_prelude()}\

<%block filter="rust_test_fn_invisible">\
${lib.test_hub(hub_type_name, comments=False)}\

// Usually you wouldn't bind this to a variable, but keep calling methods
// to setup your call.
// TODO: figoure out actual arguments ... 
// let mb = hub.${resource}().${method}(...);

// Finally, execute your call and process the result
// TODO: comment in once args are properly setup !
// mb.do()
</%block>
</%block>
pub struct ${mb_type(resource, method)}<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a ${hub_type_name}<C, NC, A>
}

impl<'a, C, NC, A> MethodBuilder for ${mb_type(resource, method)}<'a, C, NC, A> {}
</%def>