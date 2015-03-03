<%! import util as pyutil %>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="lib/util.mako"/>\
<%block filter="pyutil.markdown_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
The `${util.library_name()}` library allows access to all features of *${canonicalName}*.

<%lib:docs />
<%lib:license />