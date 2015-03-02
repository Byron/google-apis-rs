<%! import util %>\
## Create new schema with everything.
## 's' contains the schema structure from json to build
<%def name="new(s)">\
<% assert s.type == "object" %>\
<%block filter="util.rust_doc_comment">\
${doc(s)}\
</%block>
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ${s.id}\
% if 'properties' in s:
 {
% for pn, p in s.properties.iteritems():
	${p.get('description', 'no description provided') | util.rust_doc_comment}
	pub ${util.mangle_ident(pn)}: ${util.to_rust_type(s.id, pn, p)},
% endfor
}
% else:
;
% endif
</%def>

<%def name="doc(s)">\
${s.get('description', 'There is no detailed description.')}
</%def>