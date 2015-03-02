<% import util %>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="mutil" file="lib/util.mako"/>\
<%block filter="util.rust_module_doc_comment">\
<%lib:docs />\
</%block>
extern crate cmn;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;