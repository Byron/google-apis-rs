<%
	from util import (markdown_comment, new_context)
	c = new_context(resources)
%>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="lib/util.mako"/>\
<%block filter="markdown_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
The `${util.library_name()}` library allows access to all features of *${canonicalName}*.

${lib.docs(c, rust_doc=False)}
<%lib:license />