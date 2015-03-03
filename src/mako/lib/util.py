import re
import collections

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
IO_RESPONSE = 'response'
IO_REQUEST = 'request'
IO_TYPES = (IO_REQUEST, IO_RESPONSE)
INS_METHOD = 'insert'
DEL_METHOD = 'delete'

NESTED_TYPE_MARKER = 'is_nested'

# ==============================================================================
## @name Filters
# ------------------------------------------------------------------------------
## @{

# rust module doc comment filter
def rust_module_doc_comment(s):
    return re_linestart.sub('//! ', s)

# rust doc comment filter
def rust_doc_comment(s):
    return re_linestart.sub('/// ', s)

# escape each string in l with "s" and return the new list
def estr(l):
    return ['"%s"' % i for i in l]

## -- End Filters -- @}

# ==============================================================================
## @name Natural Language Utilities
# ------------------------------------------------------------------------------
## @{

# l must be a list, if it is more than one, 'and' will before last item
# l will also be coma-separtated
# Returns string
def put_and(l):
    if len(l) < 2:
        return l[0]
    return ', '.join(l[:-1]) + ' and ' + l[-1]

## -- End Natural Language Utilities -- @}


# ==============================================================================
## @name Rust TypeSystem
# ------------------------------------------------------------------------------
## @{



def nested_type_name(sn, pn):
    return sn + pn.capitalize()

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
            assert(is_nested_type_property(nt))
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
        is_pod = True
        rust_type = TYPE_MAP[t.type]
        if t.type == 'array':
            rust_type = "%s<%s>" % (rust_type, nested_type(t))
            is_pod = False
        elif t.type == 'object':
            rust_type = "%s<String, %s>" % (rust_type, nested_type(t))
            is_pod = False
        elif t.type == 'string' and 'Count' in pn:
            rust_type = 'i64'
        elif rust_type == USE_FORMAT:
            rust_type = TYPE_MAP[t.format]
        if is_pod and allow_optionals:
            return "Option<%s>" % rust_type
        return rust_type
    except KeyError as err:
        raise AssertionError("%s: Property type '%s' unknown - add new type mapping: %s" % (str(err), t.type, str(t)))
    except AttributeError as err:
        raise AssertionError("%s: unknown dict layout: %s" % (str(err), t))

# return True if this property is actually a nested type
def is_nested_type_property(t):
    return 'type' in t and t.type == 'object' and 'properties' in t

# Return True if the schema is nested
def is_nested_type(s):
    return NESTED_TYPE_MARKER in s

# return an iterator yielding fake-schemas that identify a nested type
def iter_nested_types(schemas):
    for s in schemas.values():
        if 'properties' not in s:
            continue
        for pn, p in s.properties.iteritems():
            if is_nested_type_property(p):
                ns = p.copy()
                ns.id = nested_type_name(s.id, pn)
                ns[NESTED_TYPE_MARKER] = True
                yield ns
        # end for ach property
    # end for aech schma

# Return sorted type names of all markers applicable to the given schema
def schema_markers(s, c):
    res = set()

    activities = c.sta_map.get(s.id, dict())
    if len(activities) == 0:
        res.add('Part')
    else:
        # it should have at least one activity that matches it's type to qualify for the Resource trait
        for fqan, iot in activities.iteritems():
            if activity_name_to_type_name(activity_split(fqan)[0]).lower() == s.id.lower():
                res.add('Resource')
            if IO_RESPONSE in iot:
                res.add('ResponseResult')
            if IO_REQUEST in iot:
                res.add('RequestResult')
        # end for each activity
    # end handle activites

    if is_nested_type(s):
        res.add('NestedType')

    return sorted(res)

## -- End Rust TypeSystem -- @}


# -------------------------
## @name Activity Utilities
# @{

# Returns (A, B) where
# A: { SchemaTypeName -> { fqan -> ['request'|'response', ...]}
# B: { fqan -> activity_method }
# fqan = fully qualified activity name
def build_activity_mappings(activities):
    res = dict()
    fqan = dict()
    for an, a in activities.iteritems():
        if 'methods' not in a:
            continue
        for mn, m in a.methods.iteritems():
            assert m.id not in fqan
            fqan[m.id] = m
            for in_out_type_name in IO_TYPES:
                t = m.get(in_out_type_name, None)
                if t is None:
                    continue
                tn = to_rust_type(None, None, t, allow_optionals=False)
                info = res.setdefault(tn, dict())
                io_info = info.setdefault(m.id, [])
                io_info.append(in_out_type_name)
            # end for each io type

            # handle delete/getrating/(possibly others)
            # delete: has no response or request
            # getrating: response is a 'SomethingResult', which is still related to activities name
            #            the latter is used to deduce the resource name
            an, _ = activity_split(m.id)
            tn = activity_name_to_type_name(an)
            info = res.setdefault(tn, dict())
            if m.id not in info:
                info.setdefault(m.id, [])
            # end handle other cases
        # end for each method
    # end for each activity
    return res, fqan

# return (name, method)
def activity_split(fqan):
    t = fqan.split('.')
    assert len(t) == 3
    return t[1:]

# videos -> Video
def activity_name_to_type_name(an):
    return an.capitalize()[:-1]

## -- End Activity Utilities -- @}


Context = collections.namedtuple('Context', ['sta_map', 'fqan_map'])

# Expects v to be 'v\d+', throws otherwise
def to_api_version(v):
    assert len(v) >= 2 and v[0] == 'v'
    return v[1:]

# build a full library name (non-canonical)
def library_name(name, version):
    return name + to_api_version(version)
