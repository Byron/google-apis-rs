import re
re_linestart = re.compile('^', flags=re.MULTILINE)

USE_FORMAT = 'use_format_field'
TYPE_MAP = {'boolean' : 'bool',
            'integer' : USE_FORMAT,
            'number'  : USE_FORMAT,
            'uint32'  : 'u32',
            'double'  : 'f64',
            'int32'   : 'i32',
            'array'   : 'Vec',
            'string'  : 'String',
            'object'  : 'HashMap'}
TREF = '$ref'

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


def nested_type_name(sn, pn):
    return sn + pn[:1].upper() + pn[1:]

# Make properties which are reserved keywords usable
def mangle_ident(n):
    if n == 'type':
        return n + '_'
    return n

# map a json type to an rust type
# sn = schema name
# pn = property name
# t = type dict
def to_rust_type(sn, pn, t, allow_optionals=True):
    def nested_type(nt):
        if nt.get('items', None) is not None:
            nt = nt.items
        elif nt.get('additionalProperties'):
            nt = nt.additionalProperties
        else:
            assert(is_nested_type(nt))
            # It's a nested type - we take it literally like $ref, but generate a name for the type ourselves
            # This of course assumes
            return nested_type_name(sn, pn)
        return to_rust_type(sn, pn, nt, allow_optionals=False)

    # unconditionally handle $ref types, which should point to another schema.
    if TREF in t:
        tn = t[TREF]
        if allow_optionals:
            return "Option<%s>" % tn
        return tn
    try:
        rust_type = TYPE_MAP[t.type]
        if t.type == 'array':
            rust_type = "%s<%s>" % (rust_type, nested_type(t))
        elif t.type == 'object':
            rust_type = "%s<String, %s>" % (rust_type, nested_type(t))
        elif rust_type == USE_FORMAT:
            rust_type = TYPE_MAP[t.format]
        return rust_type
    except KeyError as err:
        raise AssertionError("%s: Property type '%s' unknown - add new type mapping: %s" % (str(err), t.type, str(t)))
    except AttributeError as err:
        raise AssertionError("%s: unknown dict layout: %s" % (str(err), t))

def is_nested_type(t):
    return 'type' in t and t.type == 'object' and 'additionalProperties' not in t

# return an iterator yielding fake-schemas that identify a nested type
def iter_nested_types(schemas):
    for s in schemas.values():
        if 'properties' not in s:
            continue
        for pn, p in s.properties.iteritems():
            if is_nested_type(p):
                ns = p.copy()
                ns.id = nested_type_name(s.id, pn)
                yield ns
        # end for ach property
    # end for aech schma

