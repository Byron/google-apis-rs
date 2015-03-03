<%! import util %>\
## Create new schema with everything.
## 's' contains the schema structure from json to build
<%def name="new(s, c)">\
<% 
	assert s.type == "object" 
	markers = util.schema_markers(s, c)
%>\
<%block filter="util.rust_doc_comment">\
${doc(s, c)}\
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

% for marker_trait in markers:
impl ${marker_trait} for ${s.id} {}
% endfor
</%def>

<%def name="doc(s, c)">\
${s.get('description', 'There is no detailed description.')}
% if s.id in c.sta_map:

# Activities

This type is used in activities, which are methods you may call on this type or where this type is involved in. 
The list links the activity name, along with information about where it is used (one of ${util.put_and(list('*%s*' % t 
																							for t in util.IO_TYPES))}).

${''.join("* %s (%s)\n" % (util.activity_split(a)[1], iot and '|'.join(iot) or 'none') 
													for a, iot in c.sta_map[s.id].iteritems())}
% else:

This type is not used in any activity, and only used as *part* of another schema.
% endif
</%def>