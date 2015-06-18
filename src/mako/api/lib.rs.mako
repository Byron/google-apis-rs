<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%  
    from util import (new_context, rust_comment, rust_module_doc_comment)

    c = new_context(schemas, resources, context.get('methods'))
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

include!(concat!(env!("OUT_DIR"), "/lib.rs"));