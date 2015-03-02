import re
re_linestart = re.compile('^', flags=re.MULTILINE)

# rust module doc comment filter
def rust_module_doc_comment(s):
    return re_linestart.sub('//! ', s)

# rust doc comment filter
def rust_doc_comment(s):
    return re_linestart.sub('/// ', s)

# Expects v to be 'v\d+', throws otherwise
def to_api_version(v):
	assert len(v) >= 2 and v[0] == 'v'
	return v[1:]

# l must be a list, if it is more than one, 'and' will before last item
# l will also be coma-separtated
# Returns string
def put_and(l):
    if len(l) < 2:
        return l[0]
    return ', '.join(l[:-1]) + ' and ' + l[-1]

# escape each string in l with "s" and return the new list
def estr(l):
    return ['"%s"' % i for i in l]

# build a full library name (non-canonical)
def library_name(name, version):
    return name + to_api_version(version)
