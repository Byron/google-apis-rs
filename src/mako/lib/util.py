import re
import os
from random import (randint, random, choice, seed)
import collections
from copy import deepcopy

seed(1337)

re_linestart = re.compile('^', flags=re.MULTILINE)
re_first_4_spaces = re.compile('^ {1,4}', flags=re.MULTILINE)
re_desc_parts = re.compile("((the part (names|properties) that you can include in the parameter value are)|(supported values are ))(.*?)\.", flags=re.IGNORECASE|re.MULTILINE)

USE_FORMAT = 'use_format_field'
TYPE_MAP = {'boolean' : 'bool',
            'integer' : USE_FORMAT,
            'number'  : USE_FORMAT,
            'uint32'  : 'u32',
            'double'  : 'f64',
            'float'   : 'f32',
            'int32'   : 'i32',
            'any'     : 'String', # TODO: Figure out how to handle it. It's 'interface' in Go ...
            'int64'   : 'i64',
            'uint64'  : 'u64',
            'array'   : 'Vec',
            'string'  : 'String',
            'object'  : 'HashMap'}

RESERVED_WORDS = set(('abstract', 'alignof', 'as', 'become', 'box', 'break', 'const', 'continue', 'crate', 'do',
                      'else', 'enum', 'extern', 'false', 'final', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop',
                      'macro', 'match', 'mod', 'move', 'mut', 'offsetof', 'override', 'priv', 'pub', 'pure', 'ref',
                      'return', 'sizeof', 'static', 'self', 'struct', 'super', 'true', 'trait', 'type', 'typeof',
                      'unsafe', 'unsized', 'use', 'virtual', 'where', 'while', 'yield'))


_words = [w.strip(',') for w in "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.".split(' ')]
RUST_TYPE_RND_MAP = {'bool': lambda: str(bool(randint(0, 1))).lower(),
                     'u32' : lambda: randint(0, 100),
                     'u64' : lambda: randint(0, 100),
                     'f64' : lambda: random(),
                     'f32' : lambda: random(),
                     'i32' : lambda: randint(-101, -1),
                     'i64' : lambda: randint(-101, -1),
                     'String': lambda: '"%s"' % choice(_words),
}
TREF = '$ref'
IO_RESPONSE = 'response'
IO_REQUEST = 'request'
IO_TYPES = (IO_REQUEST, IO_RESPONSE)
INS_METHOD = 'insert'
DEL_METHOD = 'delete'

NESTED_TYPE_MARKER = 'is_nested'
SPACES_PER_TAB = 4

DELEGATE_TYPE = 'Delegate'
REQUEST_PRIORITY = 100
REQUEST_MARKER_TRAIT = 'RequestValue'
PART_MARKER_TRAIT = 'Part'
NESTED_MARKER_TRAIT = 'NestedType'
REQUEST_VALUE_PROPERTY_NAME = 'request'

PROTOCOL_TYPE_INFO = {
    'simple' : {
        'arg_name': 'stream',
        'param': 'R',
        'description': """Upload media all at once.
If the upload fails for whichever reason, all progress is lost.""",
        'default': 'fs::File',
        'where': 'io::Read',
        'suffix': '',
        'example_value': 'fs::File::open("file.ext").unwrap(), 148, "application/octet-stream".parse().unwrap()'
    },
    'resumable' : {
        'arg_name': 'resumeable_stream',
        'param': 'RS',
        'description': """Upload media in a resumeable fashion.
Even if the upload fails or is interrupted, it can be resumed for a 
certain amount of time as the server maintains state temporarily.

TODO: Write more about how delegation works in this particular case.""",
        'default': 'fs::File',
        'where': 'ReadSeek',
        'suffix': '_resumable',
        'example_value': 'fs::File::open("file.ext").unwrap(), 282, "application/octet-stream".parse().unwrap()'
    }
}

data_unit_multipliers = {
    'kb': 1024,
    'mb': 1024 ** 2,
    'gb': 1024 ** 3,
    'tb': 1024 ** 4,
    'pb': 1024 ** 5,
    '%': 1,
}

HUB_TYPE_PARAMETERS = ('C', 'NC', 'A')

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

# rust comment filter
def rust_comment(s):
    return re_linestart.sub('// ', s)

# hash-based comment filter
def hash_comment(s):
    return re_linestart.sub('# ', s)

# hides lines in rust examples, if not already hidden, or empty.
def hide_rust_doc_test(s):
    return re.sub('^[^#\n]', lambda m: '# ' + m.group(), s, flags=re.MULTILINE)

# remove the first indentation (must be spaces !)
def unindent(s):
    return re_first_4_spaces.sub('', s)

# tabs: 1 tabs is 4 spaces
def unindent_first_by(tabs):
    def unindent_inner(s):
        return re_linestart.sub(' ' * tabs * SPACES_PER_TAB, s)
    return unindent_inner

# tabs: 1 tabs is 4 spaces
def indent_all_but_first_by(tabs):
    def indent_inner(s):
        try:
            i = s.index('\n')
        except ValueError:
            f = s
            p = ''
        else:
            f = s[:i+1]
            p = s[i+1:]
        return f + re_linestart.sub(' ' * (tabs * SPACES_PER_TAB), p)
    return indent_inner

# add 4 spaces to the beginning of a line.
# useful if you have defs embedded in an unindent block - they need to counteract. 
# It's a bit itchy, but logical
def indent(s):
    return re_linestart.sub(' ' * SPACES_PER_TAB, s)

# indent by given amount of spaces
def indent_by(n):
    def indent_inner(s):
        return re_linestart.sub(' ' * n, s)
    return indent_inner

# return s, with trailing newline
def trailing_newline(s):
    if not s.endswith('\n'):
        return s + '\n'
    return s

# a rust test that doesn't run though
def rust_doc_test_norun(s):
    return "```test_harness,no_run\n%s```" % trailing_newline(s)

# a rust code block in (github) markdown
def markdown_rust_block(s):
    return "```Rust\n%s```" % trailing_newline(s)    

# wraps s into an invisible doc test function.
def rust_test_fn_invisible(s):
    return "# #[test] fn egal() {\n%s# }" % trailing_newline(s)

# markdown comments
def markdown_comment(s):
    return "<!---\n%s-->" % trailing_newline(s)

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

# ['foo', ...] with e == '*' -> ['*foo*', ...]
def enclose_in(e, l):
    return ['%s%s%s' % (e, s, e) for s in l]

def md_italic(l):
    return enclose_in('*', l)

def singular(s):
    if s.endswith('ies'):
        return s[:-3]+'y'
    if s[-1] == 's':
        return s[:-1]
    return s

def split_camelcase_s(s):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1 \2', s)
    return re.sub('([a-z0-9])([A-Z])', r'\1 \2', s1).lower()

def camel_to_under(s):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', s)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

# there are property descriptions from which parts can be extracted. Regex is based on youtube ... it's sufficiently
# easy enough to add more cases ... 
# return ['part', ...] or []
def extract_parts(desc):
    res = list()
    m = re_desc_parts.search(desc)
    if m is None:
        return res
    for part in m.groups()[-1].split(' '):
        part = part.strip(',').strip()
        if not part or part == 'and':
            continue
        res.append(part)
    return res

## -- End Natural Language Utilities -- @}


# ==============================================================================
## @name Rust TypeSystem
# ------------------------------------------------------------------------------
## @{

# Return transformed string that could make a good type name
def canonical_type_name(s):
    # can't use s.capitalize() as it will lower-case the remainder of the string
    s = s.replace(' ', '')
    return s[:1].upper() + s[1:]

def nested_type_name(sn, pn):
    return sn + canonical_type_name(pn)

# Make properties which are reserved keywords usable
def mangle_ident(n):
    n = camel_to_under(n).replace('-', '.').replace('.', '_').replace('$', '')
    if n in RESERVED_WORDS:
        return n + '_'
    return n

def _is_map_prop(p):
    return 'additionalProperties' in p

# map a json type to an rust type
# sn = schema name
# pn = property name
# t = type dict
# NOTE: In case you don't understand how this algorithm really works ... me neither - THE AUTHOR
def to_rust_type(sn, pn, t, allow_optionals=True):
    def nested_type(nt):
        if 'items' in nt:
            nt = nt.items
        elif nt.get('additionalProperties'):
            nt = nt.additionalProperties
        else:
            assert(is_nested_type_property(nt))
            # It's a nested type - we take it literally like $ref, but generate a name for the type ourselves
            return nested_type_name(sn, pn)
        return to_rust_type(sn, pn, nt, allow_optionals=False)

    def wrap_type(tn):
        if allow_optionals:
            tn = "Option<%s>" % tn
        return tn

    # unconditionally handle $ref types, which should point to another schema.
    if TREF in t:
        return wrap_type(t[TREF])
    try:
        rust_type = TYPE_MAP[t.type]
        if t.type == 'array':
            return "%s<%s>" % (rust_type, nested_type(t))
        elif t.type == 'object':
            if _is_map_prop(t):
                return "%s<String, %s>" % (rust_type, nested_type(t))
            else:
                return wrap_type(nested_type(t))
        elif t.type == 'string' and 'Count' in pn:
            rust_type = 'i64'
        elif rust_type == USE_FORMAT:
            rust_type = TYPE_MAP[t.format]
        return wrap_type(rust_type)
    except KeyError as err:
        raise AssertionError("%s: Property type '%s' unknown - add new type mapping: %s" % (str(err), t.type, str(t)))
    except AttributeError as err:
        raise AssertionError("%s: unknown dict layout: %s" % (str(err), t))

# return True if this property is actually a nested type
def is_nested_type_property(t):
    return 'type' in t and t.type == 'object' and 'properties' in t or ('items' in t and 'properties' in t.items)

# Return True if the schema is nested
def is_nested_type(s):
    return NESTED_TYPE_MARKER in s

# convert a rust-type to something that would be taken as input of a function
# even though our storage type is different
def activity_input_type(p):
    if 'input_type' in p:
        return p.input_type
    n = activity_rust_type(p, allow_optionals=False)
    if n == 'String':
        n = 'str'
    # pods are copied anyway
    elif is_pod_property(p):
        return n
    return '&%s' % n

def is_pod_property(p):
    return 'format' in p or p.get('type','') == 'boolean'

# return an iterator yielding fake-schemas that identify a nested type
# NOTE: In case you don't understand how this algorithm really works ... me neither - THE AUTHOR
def iter_nested_types(schemas):
    def iter_nested_properties(prefix, properties):
        for pn, p in properties.iteritems():
            if is_nested_type_property(p):
                ns = deepcopy(p)
                ns.id = nested_type_name(prefix, pn)
                ns[NESTED_TYPE_MARKER] = True

                # To allow us recursing arrays, we simply put items one level up
                if 'items' in p:
                    ns.update((k, deepcopy(v)) for k, v in p.items.iteritems())

                yield ns
                if 'properties' in ns:
                    for np in iter_nested_properties(prefix + canonical_type_name(pn), ns.properties):
                        yield np
            elif _is_map_prop(p):
                # it's a hash, check its type
                for np in iter_nested_properties(prefix, {pn: p.additionalProperties}):
                    yield np
            # end handle prop itself
        # end for ach property
    for s in schemas.values():
        if 'properties' not in s:
            continue
        for np in iter_nested_properties(s.id, s.properties):
            yield np
    # end for aech schma

# Return sorted type names of all markers applicable to the given schema
def schema_markers(s, c):
    res = set()

    activities = c.sta_map.get(s.id, dict())
    if len(activities) == 0:
        res.add(PART_MARKER_TRAIT)
    else:
        # it should have at least one activity that matches it's type to qualify for the Resource trait
        for fqan, iot in activities.iteritems():
            if activity_name_to_type_name(activity_split(fqan)[1]).lower() == s.id.lower():
                res.add('cmn::Resource')
            if IO_RESPONSE in iot:
                res.add('ResponseResult')
            if IO_REQUEST in iot:
                res.add(REQUEST_MARKER_TRAIT)
        # end for each activity
    # end handle activites

    if is_nested_type(s):
        res.add(NESTED_MARKER_TRAIT)

    return sorted(res)

## -- End Rust TypeSystem -- @}


# -------------------------
## @name Activity Utilities
# @{
# return (category, name, method)
def activity_split(fqan):
    t = fqan.split('.')
    return t[0], t[1], '.'.join(t[2:])

# Shorthand to get a type from parameters of activities
def activity_rust_type(p, allow_optionals=True):
    return to_rust_type(None, p.name, p, allow_optionals=allow_optionals)

# the inverse of activity-split, but needs to know the 'name' of the API
def to_fqan(name, resource, method):
    return '%s.%s.%s' % (name, resource, method)

# videos -> Video
def activity_name_to_type_name(an):
    return canonical_type_name(an)[:-1]

# yields (category, resource, activity, activity_data)
def iter_acitivities(c):
    return ((activity_split(an) + [a]) for an, a in c.fqan_map.iteritems())

# return a list of parameter structures of all params of the given method dict
# apply a prune filter to restrict the set of returned parameters.
# The order will always be: partOrder + alpha
def method_params(m, required=None, location=None):
    res = list()
    po = m.get('parameterOrder', [])
    for pn, p in m.get('parameters', dict()).iteritems():
        if required is not None and p.get('required', False) != required:
            continue
        if location is not None and p.get('location', '') != location:
            continue
        np = deepcopy(p)
        np['name'] = pn
        try:
            # po = ['part', 'foo']
            # part_prio = 2 - 0 = 2
            # foo_prio = 2 - 1 = 1
            # default = 0
            prio = len(po) - po.index(pn)
        except ValueError:
            prio = 0
        np['priority'] = prio
        res.append(np)
    # end for each parameter
    return sorted(res, key=lambda p: (p.priority, p.name), reverse=True)

def _method_io(type_name, schemas, c, m, marker=None):
    s = schemas.get(m.get(type_name, dict()).get(TREF))
    if s is None:
        return s
    if s and marker and marker not in schema_markers(s, c):
        return None
    return s

# return the given method's request or response schema (dict), or None.
# optionally return only schemas with the given marker trait
def method_request(schemas, c, m, marker=None):
    return _method_io('request', schemas, c, m, marker)

# As method request, but returns response instead
def method_response(schemas, c, m, marker=None):
    return _method_io('response', schemas, c, m, marker)

# return string like 'n.clone()', but depending on the type name of tn (e.g. &str -> n.to_string())
def rust_copy_value_s(n, tn, p):
    if 'clone_value' in p:
        return p.clone_value.format(n)
    nc = n + '.clone()'
    if tn == '&str':
        nc = n + '.to_string()'
    elif is_pod_property(p):
        nc = n
    return nc

# convert a schema into a property (for use with rust type generation).
# n = name of the property
def schema_to_required_property(s, n):
    return type(s)({'name': n, TREF: s.id, 'priority': REQUEST_PRIORITY, 'is_query_param': False})

def is_required_property(p):
    return p.get('required', False) or p.get('priority', 0) > 0

# method_params(...), request_value|None -> (required_properties, optional_properties, part_prop|None)
def organize_params(params, request_value):
    part_prop = None
    optional_props = list()
    required_props = list()
    for p in params:
        if is_required_property(p):
            if request_value and p.name == 'part':
                assert part_prop is None
                part_prop = p
            else:
                required_props.append(p)
        else:
            optional_props.append(p)
    # end for each property
    return required_props, optional_props, part_prop

# returns method parameters based on whether we can make uploads, and which protocols are supported
# or empty list if there is no media upload
def method_media_params(m):
    if not m.get('supportsMediaUpload', False):
        return []

    mu = m.get('mediaUpload')
    assert mu is not None

    # actually, one of them is required, but we can't encode that ... 
    # runtime will have to check
    res = list()
    for pn, proto in mu.protocols.iteritems():
        # the pi (proto-info) dict can be shown to the user
        pi = {'multipart': proto.multipart and 'yes' or 'no', 'maxSize': mu.get('maxSize', '0kb'), 'validMimeTypes': mu.accept}
        try:
            ti = type(m)(PROTOCOL_TYPE_INFO[pn])
        except KeyError:
            raise AssertionError("media upload protocol '%s' is not implemented" % pn)
        p = type(m)({'name': 'media_%s',
             'info': pi, 
             'path': proto.path, 
             'type': ti,
             'description': ti.description,
             'max_size': size_to_bytes(mu.get('maxSize', '0kb'))})
        res.append(p)
    # end for each proto

    return res

# Build all parameters used in a given method !
# schemas, context, method(dict), 'request'|'response', request_prop_name -> (params, request_value|None)
def build_all_params(schemas, c, m, n, npn):
    request_value = method_request(schemas, c, m)
    params = method_params(m)
    if request_value:
        params.insert(0, schema_to_required_property(request_value, npn))
    # add the delegate. It's a type parameter, which has to remain in sync with the type-parameters we actually build.
    dp = type(m)({ 'name': 'delegate',
           TREF: "&'a mut %s" % DELEGATE_TYPE, 
          'input_type': "&'a mut %s" % DELEGATE_TYPE,
          'clone_value': '{}',
          'skip_example' : True,
          'priority': 0,
          'is_query_param': False,
          'description': 
"""The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
while executing the actual API request.

It should be used to handle progress information, and to implement a certain level of resilience."""})
    params.append(dp)
    return params, request_value


## -- End Activity Utilities -- @}


Context = collections.namedtuple('Context', ['sta_map', 'fqan_map', 'rta_map', 'rtc_map'])

# return a newly build context from the given data
def new_context(resources):
    if not resources:
        return Context(dict(), dict(), dict(), dict())
    # Returns (A, B) where
    # A: { SchemaTypeName -> { fqan -> ['request'|'response', ...]}
    # B: { fqan -> activity_method_data }
    # fqan = fully qualified activity name
    def build_activity_mappings(activities, res = None, fqan = None):
        if res is None:
            res = dict()
        if fqan is None:
            fqan = dict()
        for an, a in activities.iteritems():
            if 'resources' in a:
                build_activity_mappings(a.resources, res, fqan)
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
                _, an, _ = activity_split(m.id)
                tn = activity_name_to_type_name(an)
                info = res.setdefault(tn, dict())
                if m.id not in info:
                    info.setdefault(m.id, [])
                # end handle other cases
            # end for each method
        # end for each activity
        return res, fqan
    # end utility

    sta_map, fqan_map = build_activity_mappings(resources)
    rta_map = dict()
    rtc_map = dict()
    for an in fqan_map:
        category, resource, activity = activity_split(an)
        rta_map.setdefault(resource, list()).append(activity)
        assert rtc_map.setdefault(resource, category) == category
    return Context(sta_map, fqan_map, rta_map, rtc_map)

# Expects v to be 'v\d+', throws otherwise
def to_api_version(v):
    assert len(v) >= 2
    if v.startswith('v'):
        v = v[1:]
    return v.replace('.', 'p')

# build a full library name (non-canonical)
def library_name(name, version):
    return name + to_api_version(version)

# return type name of a resource method builder, from a resource name
def rb_type(r):
    return "%sMethodsBuilder" % singular(canonical_type_name(r))

def _to_type_params_s(p):
    return '<%s>' % ', '.join(p)

# return type parameters of a the hub, ready for use in Rust code
def hub_type_params_s():
    return _to_type_params_s(HUB_TYPE_PARAMETERS)

# return a list of where statements to server as bounds for the hub.
def hub_type_bounds():
    return ['NC: hyper::net::NetworkConnector',
            "C: BorrowMut<hyper::Client<NC>> + 'a",
            'A: oauth2::GetToken']

# return list of type bounds required by method builder
def mb_type_bounds():
    return hub_type_bounds()

_rb_type_params = ("'a", ) + HUB_TYPE_PARAMETERS


# type parameters for a resource builder - keeps hub as borrow
def rb_type_params_s(resource, c):
    return _to_type_params_s(_rb_type_params)

# type params for the given method builder, as string suitable for Rust code
def mb_type_params_s(m):
    return _to_type_params_s(_rb_type_params)

# as rb_additional_type_params, but for an individual method, as seen from a resource builder !
def mb_additional_type_params(m):
    return []

# return type name for a method on the given resource
def mb_type(r, m):
    return "%s%sMethodBuilder" % (singular(canonical_type_name(r)), dot_sep_to_canonical_type_name(m))

def hub_type(schemas, canonicalName):
    name = canonical_type_name(canonicalName)
    if schemas and name in schemas:
        name += 'Hub'
    return name

# return e + d[n] + e + ' ' or ''
def get_word(d, n, e = ''):
    if n in d:
        v = e + d[n] + e
        if not v.endswith(' '):
            v += ' '
        return v
    else:
        return ''

# n = 'FooBar' -> _foo_bar
def property(n):
    return '_' + mangle_ident(n)

# n = 'foo.bar.Baz' -> 'FooBarBaz'
def dot_sep_to_canonical_type_name(n):
    return ''.join(canonical_type_name(singular(t)) for t in n.split('.'))

# Convert a scope url to a nice enum variant identifier, ready for use in code
# name = name of the api, without version
def scope_url_to_variant(name, url, fully_qualified=True):
    FULL = 'Full'
    fqvn = lambda n: fully_qualified and 'Scope::%s' % n or n
    repl = lambda n: n.replace('-', '.').replace('_', '.')

    if url.endswith('/'):
        url = name[:-1]
    base = os.path.basename(url)

    assert base, name
    # special case, which works for now ... https://mail.gmail.com
    # NO can do ! Must play safe here ... 
    if not base.startswith(name):
        return fqvn(dot_sep_to_canonical_type_name(repl(base)))
    base = base[len(name):]
    base = base.strip('-').strip('.')
    if len(base) == 0:
        return fqvn(FULL)
    return fqvn(dot_sep_to_canonical_type_name(repl(base)))


# given a rust type-name (no optional, as from to_rust_type), you will get a suitable random default value
# as string suitable to be passed as reference (or copy, where applicable)
def rnd_arg_val_for_type(tn):
    try:
        return str(RUST_TYPE_RND_MAP[tn]())
    except KeyError:
        return '&Default::default()'

# Converts a size to the respective integer
# size string like 1MB or 2TB, or 35.5KB
def size_to_bytes(size):
    unit = size[-2:].lower()
    if unit[0] in '0123456789':
        assert unit[1] in '0123456789'
        return int(size)
    # end handle no unit
    try:
        return int(data_unit_multipliers[unit] * float(size[:-2]))
    except KeyError:
        raise ValueError("Invalid unit: '%s'" % unit)
    # end handle errors gracefully
