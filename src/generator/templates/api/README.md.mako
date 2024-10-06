<%
    from generator.lib.util import (markdown_comment, new_context)

    c = new_context(schemas, resources)
%>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="../../lib/util.mako"/>\
<%block filter="markdown_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
The `${util.crate_name()}` library allows access to all features of the *Google ${util.canonical_name()}* service.

${lib.docs(c, rust_doc=False)}
<%lib:license />
