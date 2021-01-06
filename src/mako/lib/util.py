import re
import os
from random import (randint, random, choice, seed)
import collections
from copy import deepcopy
import subprocess

seed(1337)

re_linestart = re.compile('^', flags=re.MULTILINE)
re_spaces_after_newline = re.compile('^ {4}', flags=re.MULTILINE)
re_first_4_spaces = re.compile('^ {1,4}', flags=re.MULTILINE)
re_desc_parts = re.compile("((the part (names|properties) that you can include in the parameter value are)|(supported values are ))(.*?)\.", flags=re.IGNORECASE|re.MULTILINE)

re_find_replacements = re.compile("\{[/\+]?\w+\*?\}")

HTTP_METHODS = set(("OPTIONS", "GET", "POST", "PUT", "DELETE", "HEAD", "TRACE", "CONNECT", "PATCH" ))

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

words = [w.strip(',') for w in "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.".split(' ')]
RUST_TYPE_RND_MAP = {'bool': lambda: str(bool(randint(0, 1))).lower(),
                     'u32' : lambda: randint(0, 100),
                     'u64' : lambda: randint(0, 100),
                     'f64' : lambda: random(),
                     'f32' : lambda: random(),
                     'i32' : lambda: randint(-101, -1),
                     'i64' : lambda: randint(-101, -1),
                     'String': lambda: '"%s"' % choice(words),
}
TREF = '$ref'
IO_RESPONSE = 'response'
IO_REQUEST = 'request'
IO_TYPES = (IO_REQUEST, IO_RESPONSE)
INS_METHOD = 'insert'
DEL_METHOD = 'delete'
METHODS_RESOURCE = 'methods'

ADD_PARAM_FN = 'param'
ADD_SCOPE_FN = 'add_scope'
ADD_PARAM_MEDIA_EXAMPLE = "." + ADD_PARAM_FN + '("alt", "media")'

SPACES_PER_TAB = 4

NESTED_TYPE_SUFFIX = 'item'
DELEGATE_TYPE = 'client::Delegate'
REQUEST_PRIORITY = 100
REQUEST_MARKER_TRAIT = 'client::RequestValue'
RESPONSE_MARKER_TRAIT = 'client::ResponseResult'
RESOURCE_MARKER_TRAIT = 'client::Resource'
CALL_BUILDER_MARKERT_TRAIT = 'client::CallBuilder'
METHODS_BUILDER_MARKER_TRAIT = 'client::MethodsBuilder'
PART_MARKER_TRAIT = 'client::Part'
NESTED_MARKER_TRAIT = 'client::NestedType'
REQUEST_VALUE_PROPERTY_NAME = 'request'
DELEGATE_PROPERTY_NAME = 'delegate'
TO_PARTS_MARKER = 'client::ToParts'
UNUSED_TYPE_MARKER = 'client::UnusedType'

PROTOCOL_TYPE_INFO = {
    'simple' : {
        'arg_name': 'stream',
        'description': """Upload media all at once.
If the upload fails for whichever reason, all progress is lost.""",
        'default': 'fs::File',
        'suffix': '',
        'example_value': 'fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()'
    },
    'resumable' : {
        'arg_name': 'resumeable_stream',
        'description': """Upload media in a resumable fashion.
Even if the upload fails or is interrupted, it can be resumed for a
certain amount of time as the server maintains state temporarily.

The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
`cancel_chunk_upload(...)`.""",
        'default': 'fs::File',
        'suffix': '_resumable',
        'example_value': 'fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()'
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

HUB_TYPE_PARAMETERS = ('C', 'A')

def items(p):
    if isinstance(p, dict):
        return p.items()
    else:
        return p._items()

def custom_sorted(p):
    if not isinstance(p, list):
        assert(false, p, "unexpected type")
    return sorted(p, key = lambda p: p['name'])

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

# returns true if there is an indication for something that is interpreted as doc comment by rustdoc
def has_markdown_codeblock_with_indentation(s):
    return re_spaces_after_newline.search(s) != None

def preprocess(s):
    p = subprocess.Popen([os.environ['PREPROC']], close_fds=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    res = p.communicate(s.encode('utf-8'))
    return res[0].decode('utf-8')

# runs the preprocessor in case there is evidence for code blocks using indentation
def rust_doc_sanitize(s):
    if has_markdown_codeblock_with_indentation(s):
        return preprocess(s)
    else:
        return s

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

# don't do anything with the passed in string
def pass_through(s):
    return s

# tabs: 1 tabs is 4 spaces
def unindent_first_by(tabs):
    def unindent_inner(s):
        return re_linestart.sub(' ' * tabs * SPACES_PER_TAB, s)
    return unindent_inner

# filter to remove empty lines from a string
def remove_empty_lines(s):
    return re.sub("^\n", '', s, flags=re.MULTILINE)

# Prepend prefix  to each line but the first
def prefix_all_but_first_with(prefix):
    def indent_inner(s):
        try:
            i = s.index('\n')
        except ValueError:
            f = s
            p = None
        else:
            f = s[:i+1]
            p = s[i+1:]
        if p is None:
            return f
        return f + re_linestart.sub(prefix, p)
    return indent_inner


# tabs: 1 tabs is 4 spaces
def indent_all_but_first_by(indent, indent_in_tabs=True):
    if indent_in_tabs:
        indent *= SPACES_PER_TAB
    spaces = ' ' * indent
    return prefix_all_but_first_with(spaces)

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

# escape all '"' with '\"'
def escape_rust_string(s):
    return s.replace('"', '\\"')

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

def capitalize(s):
    return s[:1].upper() + s[1:]

# Return transformed string that could make a good type name
def canonical_type_name(s):
    # can't use s.capitalize() as it will lower-case the remainder of the string
    s = ''.join(capitalize(t) for t in s.split(' '))
    s = ''.join(capitalize(t) for t in s.split('_'))
    s = ''.join(capitalize(t) for t in s.split('-'))
    return capitalize(s)

def nested_type_name(sn, pn):
    suffix = canonical_type_name(pn)
    return sn + suffix

# Make properties which are reserved keywords usable
def mangle_ident(n):
    n = camel_to_under(n).replace('-', '.').replace('.', '_').replace('$', '')
    if n in RESERVED_WORDS:
        return n + '_'
    return n

def is_map_prop(p):
    return 'additionalProperties' in p

def _assure_unique_type_name(schemas, tn):
    if tn in schemas:
        tn += 'Nested'
        assert tn not in schemas
    return tn

# map a json type to an rust type
# sn = schema name
# pn = property name
# t = type dict
# NOTE: In case you don't understand how this algorithm really works ... me neither - THE AUTHOR
def to_rust_type(schemas, sn, pn, t, allow_optionals=True, _is_recursive=False):
    def nested_type(nt):
        if 'items' in nt:
            nt = nt['items']
        elif 'additionalProperties' in nt:
            nt = nt['additionalProperties']
        else:
            assert(is_nested_type_property(nt))
            # It's a nested type - we take it literally like $ref, but generate a name for the type ourselves
            return _assure_unique_type_name(schemas, nested_type_name(sn, pn))
        return to_rust_type(schemas, sn, pn, nt, allow_optionals=False, _is_recursive=True)

    def wrap_type(tn):
        if allow_optionals:
            tn = "Option<%s>" % tn
        return tn

    # unconditionally handle $ref types, which should point to another schema.
    if TREF in t:
        # simple, non-recursive fix for some recursive types. This only works on the first depth level
        # which is fine for now. 'allow_optionals' implicitly restricts type boxing for simple types - it
        # usually is on on the first call, and off when recursion is involved.
        tn = t[TREF]
        if not _is_recursive and tn == sn:
            tn = 'Option<Box<%s>>' % tn
        return wrap_type(tn)
    try:
        rust_type = TYPE_MAP[t['type']]
        if t['type'] == 'array':
            return wrap_type("%s<%s>" % (rust_type, (nested_type(t))))
        elif t['type'] == 'object':
            if is_map_prop(t):
                return wrap_type("%s<String, %s>" % (rust_type, nested_type(t)))
            else:
                return wrap_type(nested_type(t))
        elif rust_type == USE_FORMAT:
            rust_type = TYPE_MAP[t['format']]

        if t.get('repeated', False):
            rust_type = 'Vec<%s>' % rust_type
        else:
            rust_type = wrap_type(rust_type)
        return rust_type
    except KeyError as err:
        raise AssertionError("%s: Property type '%s' unknown - add new type mapping: %s" % (str(err), t['type'], str(t)))
    except AttributeError as err:
        raise AssertionError("%s: unknown dict layout: %s" % (str(err), t))

# return True if this property is actually a nested type
def is_nested_type_property(t):
    return 'type' in t and t['type'] == 'object' and 'properties' in t or ('items' in t and 'properties' in t['items'])

# Return True if the schema is nested
def is_nested_type(s):
    return len(s.parents) > 0

# convert a rust-type to something that would be taken as input of a function
# even though our storage type is different
def activity_input_type(schemas, p):
    if 'input_type' in p:
        return p.input_type
    n = activity_rust_type(schemas, p, allow_optionals=False)
    if n == 'String':
        n = 'str'
    # pods are copied anyway
    elif is_pod_property(p) or p.get(TREF):
        return n
    return '&%s' % n

def is_pod_property(p):
    return 'format' in p or p.get('type','') == 'boolean'


def _traverse_schema_ids(s, c):
    ids = [s.id]
    used_by = s.used_by + s.parents

    seen = set() # protect against loops, just to be sure ...
    while used_by:
        id = used_by.pop()
        if id in seen:
            continue
        seen.add(id)
        ids.append(id)

        oid = c.schemas[id]
        used_by.extend(oid.used_by)
        used_by.extend(oid.parents)
    # end gather usages
    return ids

# Return sorted type names of all markers applicable to the given schema
# This list is transitive. Thus, if the schema is used as child of someone with a trait, it
# inherits this trait
def schema_markers(s, c, transitive=True):
    res = set()
    ids = transitive and _traverse_schema_ids(s, c) or [s.id]

    has_activity = False
    for sid in ids:
        activities = c.sta_map.get(sid, dict())
        if len(activities) == 0:
            continue
        has_activity = True
        # it should have at least one activity that matches it's type to qualify for the Resource trait
        for fqan, iot in activities.items():
            _, resource, _ = activity_split(fqan)
            if resource and activity_name_to_type_name(resource).lower() == sid.lower():
                res.add(RESOURCE_MARKER_TRAIT)
            m = c.fqan_map[to_fqan(*activity_split(fqan))]
            params, _ = build_all_params(c, m)
            part_prop, _ = parts_from_params(params)
            if part_prop is not None and 'properties' in s:
                res.add(TO_PARTS_MARKER)
            if IO_RESPONSE in iot:
                res.add(RESPONSE_MARKER_TRAIT)
            if IO_REQUEST in iot:
                res.add(REQUEST_MARKER_TRAIT)
        # end for each activity
        # end handle activites
    # end for each parent ... transitively

    if is_nested_type(s):
        res.add(NESTED_MARKER_TRAIT)
    # if len(s.used_by) + len(s.parents) > 0:
    if len(c.sta_map.get(s.id, dict())) == 0:
        res.add(PART_MARKER_TRAIT)

    if not has_activity:
        res.add(UNUSED_TYPE_MARKER)

    return sorted(res)

## -- End Rust TypeSystem -- @}

# NOTE: unfortunately, it turned out that sometimes fields are missing. The only way to handle this is to
# use optionals everywhere. If that should ever change, we can make a decision here based on the
# non-transitive markers that we get here !
def is_schema_with_optionals(schema_markers):
    return True

# -------------------------
## @name Activity Utilities
# @{
# return (category, name|None, method)
def activity_split(fqan):
    t = fqan.split('.')
    mt = t[2:]
    if not mt:
        # make this the method, with not resource
        mt = [t[1]]
        t[1] = METHODS_RESOURCE
    # end
    return t[0], t[1], '.'.join(mt)

# Shorthand to get a type from parameters of activities
def activity_rust_type(schemas, p, allow_optionals=True):
    return to_rust_type(schemas, None, p.name, p, allow_optionals=allow_optionals)

# the inverse of activity-split, but needs to know the 'name' of the API
def to_fqan(name, resource, method):
    return '%s.%s.%s' % (name, resource, method)

# videos -> Video
def activity_name_to_type_name(an):
    return canonical_type_name(an)[:-1]

# yields (category, resource, activity, activity_data)
def iter_acitivities(c):
    return ((activity_split(an) + [a]) for an, a in c.fqan_map.items())

# return a list of parameter structures of all params of the given method dict
# apply a prune filter to restrict the set of returned parameters.
# The order will always be: partOrder + alpha
def _method_params(m, required=None, location=None):
    res = list()
    po = m.get('parameterOrder', [])
    for pn, p in m.get('parameters', dict()).items():
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

def _method_io(type_name, c, m, marker=None):
    s = c.schemas.get(m.get(type_name, dict()).get(TREF))
    if s is None:
        return s
    if s and marker and marker not in schema_markers(s, c):
        return None
    return s

# return the given method's request or response schema (dict), or None.
# optionally return only schemas with the given marker trait
def method_request(c, m, marker=None):
    return _method_io('request', c, m, marker)

# As method request, but returns response instead
def method_response(c, m, marker=None):
    return _method_io('response', c, m, marker)

# return string like 'n.clone()', but depending on the type name of tn (e.g. &str -> n.to_string())
def rust_copy_value_s(n, tn, p):
    if 'clone_value' in p:
        return p.clone_value.format(n)
    nc = n + '.clone()'
    if tn == '&str':
        nc = n + '.to_string()'
    elif is_pod_property(p) or p.get(TREF):
        nc = n
    return nc

# convert a schema into a property (for use with rust type generation).
# n = name of the property
def schema_to_required_property(s, n):
    return type(s)({'name': n, TREF: s.id, 'priority': REQUEST_PRIORITY, 'is_query_param': False})

def is_required_property(p):
    return p.get('required', False) or p.get('priority', 0) > 0

def is_repeated_property(p):
    return p.get('repeated', False)

def setter_fn_name(p):
    fn_name = p.name
    if is_repeated_property(p):
        fn_name = 'add_' + fn_name
    return fn_name

# _method_params(...), request_value|None -> (required_properties, optional_properties, part_prop|None)
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
    for pn, proto in mu.protocols.items():
        # the pi (proto-info) dict can be shown to the user
        pi = {'multipart': proto.multipart and 'yes' or 'no', 'maxSize': mu.get('maxSize', '0kb'), 'validMimeTypes': mu.accept}
        try:
            ti = type(m)(PROTOCOL_TYPE_INFO[pn])
        except KeyError:
            raise AssertionError("media upload protocol '%s' is not implemented" % pn)
        p = type(m)({'name': 'media_%s',
             'info': pi,
             'protocol': pn,
             'path': proto.path,
             'type': ti,
             'description': ti.description,
             'max_size': size_to_bytes(mu.get('maxSize', '0kb'))})
        res.append(p)
    # end for each proto

    return res

# Build all parameters used in a given method !
# schemas, context, method(dict), 'request'|'response', request_prop_name -> (params, request_value|None)
def build_all_params(c, m):
    request_value = method_request(c, m)
    params = _method_params(m)
    if request_value:
        params.insert(0, schema_to_required_property(request_value, REQUEST_VALUE_PROPERTY_NAME))
    # add the delegate. It's a type parameter, which has to remain in sync with the type-parameters we actually build.
    dp = type(m)({ 'name': DELEGATE_PROPERTY_NAME,
           TREF: "&'a mut dyn %s" % DELEGATE_TYPE,
          'input_type': "&'a mut dyn %s" % DELEGATE_TYPE,
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


Context = collections.namedtuple('Context', ['sta_map', 'fqan_map', 'rta_map', 'rtc_map', 'schemas'])

# return a newly build context from the given data
def new_context(schemas, resources, methods):
    # Returns (A, B) where
    # A: { SchemaTypeName -> { fqan -> ['request'|'response', ...]}
    # B: { fqan -> activity_method_data }
    # fqan = fully qualified activity name
    def build_activity_mappings(activities, res = None, fqan = None):
        if res is None:
            res = dict()
        if fqan is None:
            fqan = dict()
        for k,a in activities.items():
            if 'resources' in a:
                build_activity_mappings(a.resources, res, fqan)
            if 'methods' not in a:
                continue
            for mn, m in a.methods.items():
                assert m.id not in fqan
                category, resource, method = activity_split(m.id)
                # This may be another name by which people try to find the method.
                # As it has no resource, we put in a 'fake resource' (METHODS_RESOURCE), which
                # needs some special treatment only in key-spots
                fqan_key = m.id
                if resource == METHODS_RESOURCE:
                    fqan_key = to_fqan(category, resource, method)
                fqan[fqan_key] = m
                for in_out_type_name in IO_TYPES:
                    t = m.get(in_out_type_name, None)
                    if t is None:
                        continue
                    tn = to_rust_type(schemas, None, None, t, allow_optionals=False)
                    info = res.setdefault(tn, dict())
                    io_info = info.setdefault(m.id, [])
                    io_info.append(in_out_type_name)
                # end for each io type

                # handle delete/getrating/(possibly others)
                # delete: has no response or request
                # getrating: response is a 'SomethingResult', which is still related to activities name
                #            the latter is used to deduce the resource name
                tn = activity_name_to_type_name(resource)
                info = res.setdefault(tn, dict())
                if m.id not in info:
                    info.setdefault(m.id, [])
                # end handle other cases
            # end for each method
        # end for each activity
        return res, fqan
    # end utility

    # A dict of {s.id -> schema} , with all schemas having the 'parents' key set with [s.id, ...] of all parents
    # in order of traversal, [-1] is first parent, [0] is the root of them all
    def build_schema_map():
        # 'type' in t and t.type == 'object' and 'properties' in t or ('items' in t and 'properties' in t.items)
        PARENT = 'parents'
        USED_BY = 'used_by'
        def assure_list(s, k):
            if k not in s:
                s[k] = list()
            return s[k]
        # end
        def link_used(s, rs):
            if TREF in s:
                l = assure_list(all_schemas[s[TREF]], USED_BY)
                if rs.id not in l:
                    l.append(rs.id)

        def append_unique(l, s):
            if s not in l:
                l.append(s)
            return l

        all_schemas = deepcopy(schemas)
        def recurse_properties(prefix, rs, s, parent_ids):
            assure_list(s, USED_BY)
            assure_list(s, PARENT).extend(parent_ids)
            link_used(s, rs)

            if is_nested_type_property(s) and 'id' not in s:
                s.id = prefix
                all_schemas[s.id] = s
                rs = s
            # end this is already a perfectly valid type

            properties = s.get('properties', {'': s})

            for pn, p in items(properties):
                link_used(p, rs)
                if is_nested_type_property(p):
                    ns = deepcopy(p)
                    ns.id = _assure_unique_type_name(schemas, nested_type_name(prefix, pn))
                    all_schemas[ns.id] = ns

                    # To allow us recursing arrays, we simply put items one level up
                    if 'items' in p:
                        ns.update((k, deepcopy(v)) for k, v in p.items.items())

                    recurse_properties(ns.id, ns, ns, append_unique(parent_ids, rs.id))
                elif is_map_prop(p):
                    recurse_properties(nested_type_name(prefix, pn), rs,
                                                        p.additionalProperties, append_unique(parent_ids, rs.id))
                elif 'items' in p:
                    recurse_properties(nested_type_name(prefix, pn), rs,
                                                        p.items, append_unique(parent_ids, rs.id))
                elif 'variant' in p:
                    for enum in p.variant.map:
                        recurse_properties(prefix, rs, enum, parent_ids)
                # end handle prop itself
            # end for each property
        # end utility
        for s in all_schemas.values():
            recurse_properties(s.id, s, s, [])
        # end for each schema

        return all_schemas
    # end utility

    all_schemas = schemas and build_schema_map() or dict()
    if not (resources or methods):
        return Context(dict(), dict(), dict(), dict(), all_schemas)

    rta_map, rtc_map, sta_map, fqan_map = dict(), dict(), dict(), dict()

    sources = list()
    if bool(resources):
        sources.append(resources)
    if bool(methods):
        sources.append({None : type(methods)({'methods' : methods})})

    for data_source in sources:
        _sta_map, _fqan_map = build_activity_mappings(data_source)
        for an in _fqan_map:
            category, resource, activity = activity_split(an)
            rta_map.setdefault(resource, list()).append(activity)
            assert rtc_map.setdefault(resource, category) == category
        # end for each fqan
        sta_map.update(_sta_map)
        fqan_map.update(_fqan_map)
    # end for each data source
    return Context(sta_map, fqan_map, rta_map, rtc_map, all_schemas)

def _is_special_version(v):
    return v.endswith('alpha') or v.endswith('beta')

def to_api_version(v):
    m = re.search("_?v(\d(\.\d)*)_?", v)
    if not m and _is_special_version(v):
        return v
    assert m, "Expected to find a version within '%s'" % v

    # skip trailing zeroes
    tokens = m.group(1).split('.')
    up_to = len(tokens)
    for t in reversed(tokens[1:]):
        if t == '0':
            up_to -= 1
        else:
            break

    version = 'd'.join(tokens[:up_to])
    remainder = v.replace(m.group(0), '')
    if remainder:
        version = version + '_' + remainder
    return version

def normalize_library_name(name):
    return name.lower()

# build a full library name (non-canonical)
def library_name(name, version):
    version = to_api_version(version)

    if name[-1].isdigit():
        name += '_'
        if not _is_special_version(version):
            version = 'v' + version
    return normalize_library_name(name) + version

def target_directory_name(name, version, suffix):
    return library_name(name, version) + suffix

# return crate name for given result of `library_name()`
def library_to_crate_name(name, suffix=''):
    return 'google-' + name + suffix

# return version like 0.1.0+2014031421
def crate_version(build_version, revision):
    return '%s+%s' % (build_version, isinstance(revision, str) and revision or '00000000')

# return a crate name for us in extern crate statements
def to_extern_crate_name(crate_name):
    return crate_name.replace('-', '_')

def docs_rs_url(base_url, crate_name, version):
    return base_url + '/' + crate_name + '/' + version

def crate_name(name, version, make):
    return library_to_crate_name(library_name(name, version), make.target_suffix)

def gen_crate_dir(name, version, ti):
    return to_extern_crate_name(library_to_crate_name(library_name(name, version), ti.target_suffix))

def crates_io_url(name, version):
    return "https://crates.io/crates/%s" % library_to_crate_name(library_name(name, version))

def program_name(name, version):
    return library_name(name, version).replace('_', '-')

def api_json_path(api_base, name, version):
    return api_base + '/' + name + '/' + version + '/' + name + '-api.json'

def api_index(DOC_ROOT, name, version, ti, cargo, revision, check_exists=True):
    crate_dir = gen_crate_dir(name, version, ti)
    if ti.documentation_engine == 'rustdoc':
        semver = crate_version(cargo.build_version, revision)
        index_file_path = docs_rs_url(cargo.doc_base_url, crate_name(name, version, ti), semver)
        return index_file_path
    else:
        index_file_path = crate_dir + '/' + 'index.html'
        if not check_exists or os.path.isfile(os.path.join(DOC_ROOT, index_file_path)):
            return index_file_path
        return None

# return type name of a resource method builder, from a resource name
def rb_type(r):
    return "%sMethods" % singular(canonical_type_name(r))

def _to_type_params_s(p):
    return '<%s>' % ', '.join(p)

# return type parameters of a the hub, ready for use in Rust code
def hub_type_params_s():
    return _to_type_params_s(HUB_TYPE_PARAMETERS)

# return a list of where statements to server as bounds for the hub.
def hub_type_bounds():
    return ['C: BorrowMut<hyper::Client>',
            'A: oauth2::GetToken']

# Returns True if this API has particular authentication scopes to choose from
def supports_scopes(auth):
    return bool(auth) and bool(auth.oauth2)

# Returns th desired scope for the given method. It will use read-only scopes for read-only methods
# May be None no scope-based authentication is required
def method_default_scope(m):
    if 'scopes' not in m:
        return None
    default_scope = sorted(m.scopes)[0]
    if m.httpMethod in ('HEAD', 'GET', 'OPTIONS', 'TRACE'):
        for scope in m.scopes:
            if 'readonly' in scope:
                default_scope = scope
                break
        # end for each scope
    # end try to find read-only default scope
    return default_scope

# return list of type bounds required by method builder
def mb_type_bounds():
    return hub_type_bounds()

_rb_type_params = ("'a", ) + HUB_TYPE_PARAMETERS


# type parameters for a resource builder - keeps hub as borrow
def rb_type_params_s(resource, c):
    return _to_type_params_s(_rb_type_params)

# type bounds for resource and method builder
def struct_type_bounds_s():
    return ', '.join(tp + ": 'a" for tp in HUB_TYPE_PARAMETERS)

# type params for the given method builder, as string suitable for Rust code
def mb_type_params_s(m):
    return _to_type_params_s(_rb_type_params)

# as rb_additional_type_params, but for an individual method, as seen from a resource builder !
def mb_additional_type_params(m):
    return []

# return type name for a method on the given resource
def mb_type(r, m):
    return "%s%sCall" % (singular(canonical_type_name(r)), dot_sep_to_canonical_type_name(m))

# canonicalName = util.canonical_name()
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

def upload_action_fn(upload_action_term, suffix):
    return upload_action_term + suffix

# n = 'foo.bar.Baz' -> 'FooBarBaz'
def dot_sep_to_canonical_type_name(n):
    return ''.join(canonical_type_name(singular(t)) for t in n.split('.'))

def find_fattest_resource(c):
    fr = None
    if c.schemas:
        for candidate in sorted(c.schemas.values(),
                            key=lambda s: (len(c.sta_map.get(s.id, [])), len(s.get('properties', []))), reverse=True):
            if candidate.id in c.sta_map:
                fr = candidate
                break
        # end for each candidate to check
    # end if there are schemas
    return fr

# Extract valid parts from the description of the parts prop contained within the given parameter list
# can be an empty list.
def parts_from_params(params):
    part_prop = None
    for p in params:
        if p.name == 'part':
            part_prop = p
            break
    # end for each param
    if part_prop:
        return part_prop, extract_parts(part_prop.get('description', ''))
    return part_prop, list()

# Convert a scope url to a nice enum variant identifier, ready for use in code
# name = name of the api, without version, non-normalized (!)
def scope_url_to_variant(name, url, fully_qualified=True):
    name = normalize_library_name(name)
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

def method_name_to_variant(name):
    fmt = 'hyper::method::Method::Extension("%s")'
    if name in HTTP_METHODS:
        name = name.capitalize()
        fmt = 'hyper::method::Method::%s'
    return fmt % name.capitalize()

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


if __name__ == '__main__':
    raise AssertionError('For import only')
