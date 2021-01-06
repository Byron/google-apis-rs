import util

import os
import re
import collections
from copy import deepcopy
from random import (randint, random, choice)

SPLIT_START = '>>>>>>>'
SPLIT_END = '<<<<<<<'

PARAM_FLAG = 'p'
STRUCT_FLAG = 'r'
UPLOAD_FLAG = 'u'
OUTPUT_FLAG = 'o'
VALUE_ARG = 'v'
KEY_VALUE_ARG = 'kv'
SCOPE_FLAG = 'scope'
CONFIG_DIR_FLAG = 'config-dir'
DEBUG_FLAG = 'debug'
DEBUG_AUTH_FLAG = 'debug-auth'
DEFAULT_MIME = 'application/octet-stream'

MODE_ARG = 'mode'
FILE_ARG = 'file'
FILE_FLAG = 'f'
MIME_ARG = 'mime'
MIME_FLAG = 'm'
OUT_ARG = 'out'

SCOPE_ARG = 'url'
CONFIG_DIR_ARG = 'folder'

FIELD_SEP = '.'

CONFIG_DIR = '~/.google-service-cli'

POD_TYPES = set(('boolean', 'integer', 'number', 'uint32', 'double', 'float', 'int32', 'int64', 'uint64', 'string'))

re_splitters = re.compile(r"%s ([\w\-\.]+)\n(.*?)\n%s" % (SPLIT_START, SPLIT_END), re.MULTILINE|re.DOTALL)

MethodContext = collections.namedtuple('MethodContext', ['m', 'response_schema', 'params', 'request_value',
                                                         'media_params' ,'required_props', 'optional_props',
                                                         'part_prop'])

CTYPE_POD = 'pod'
CTYPE_ARRAY = 'list'
CTYPE_MAP = 'map'
SchemaEntry = collections.namedtuple('SchemaEntry', ['container_type', 'actual_property', 'property'])

JSON_TYPE_RND_MAP = {'boolean': lambda: str(bool(randint(0, 1))).lower(),
                     'integer' : lambda: randint(0, 100),
                     'uint32' : lambda: randint(0, 100),
                     'uint64' : lambda: randint(0, 65556),
                     'float' : lambda: random(),
                     'double' : lambda: random(),
                     'number' : lambda: random(),
                     'int32' : lambda: randint(-101, -1),
                     'int64' : lambda: randint(-101, -1),
                     'string': lambda: '%s' % choice(util.words).lower()}

JSON_TYPE_TO_ENUM_MAP = {'boolean' : 'Boolean',
                         'integer' : 'Int',
                         'number'  : 'Float',
                         'uint32'  : 'Int',
                         'double'  : 'Float',
                         'float'   : 'Float',
                         'int32'   : 'Int',
                         'any'     : 'String', # TODO: Figure out how to handle it. It's 'interface' in Go ...
                         'int64'   : 'Int',
                         'uint64'  : 'Uint',
                         'string'  : 'String'}

CTYPE_TO_ENUM_MAP = {CTYPE_POD:   'Pod',
                     CTYPE_ARRAY: 'Vec',
                     CTYPE_MAP:   'Map'}

JSON_TYPE_VALUE_MAP = {'boolean': 'false',
                       'integer' : '-0',
                       'uint32' : '0',
                       'uint64' : '0',
                       'float' : '0.0',
                       'double' : '0.0',
                       'number' : '0.0',
                       'int32' : '-0',
                       'int64' : '-0',
                       'string': ''}

assert len(set(JSON_TYPE_RND_MAP.keys()) ^ POD_TYPES) == 0

def new_method_context(resource, method, c):
    m = c.fqan_map[util.to_fqan(c.rtc_map[resource], resource, method)]
    response_schema = util.method_response(c, m)
    params, request_value = util.build_all_params(c, m)
    media_params = util.method_media_params(m)
    required_props, optional_props, part_prop = util.organize_params(params, request_value)

    return MethodContext(m, response_schema, params, request_value, media_params,
                         required_props, optional_props, part_prop)

def comma_sep_fields(fields):
    return ', '.join('"%s"' % mangle_subcommand(f) for f in fields)

# Returns a string representing a string-vector of mangled names
# fields is an iterator
def field_vec(fields):
    return "vec![%s]" % comma_sep_fields(fields)

def pretty(n):
    return ' '.join(s.capitalize() for s in mangle_subcommand(n).split('-'))


def is_request_value_property(mc, p):
    return mc.request_value and mc.request_value.id == p.get(util.TREF)


# transform name to be a suitable subcommand
def mangle_subcommand(name):
    return util.camel_to_under(name).replace('_', '-').replace('.', '-')

def ident(name):
    return mangle_subcommand(name).replace('-', '_')

# Return a required value in Rust, using unwrap()
def req_value(name):
    return 'opt.value_of("' + mangle_subcommand(name) + '").unwrap()'

def opt_value(name, opt='opt', default=''):
    return opt + '.value_of("' + mangle_subcommand(name) + ('").unwrap_or("%s")' % default)

def opt_values(name, opt='opt'):
    return opt + '.values_of("' + mangle_subcommand(name) + '").map(|i|i.collect()).unwrap_or(Vec::new()).iter()'

def application_secret_path(program_name):
    return program_name + '-secret.json'

# Returns identifier for method dealing with options for the given resource-method pair
def call_method_ident(resource, method):
    return '_%s_%s' % (ident(resource), ident(method))

# transform the resource name into a suitable filename to contain the markdown documentation for it
def subcommand_md_filename(resource, method):
    return mangle_subcommand(resource) + '_' + mangle_subcommand(method) + '.md'

def docopt_mode(protocols):
    mode = '|'.join(protocols)
    if len(protocols) > 1:
        mode = '(%s)' % mode
    return mode

# Returns a possibly remapped type, based on its name.
# Useful to map strings to more suitable types, i.e. counts
def actual_json_type(name, type):
    return type

# return a string representing property `p` suitable for docopt argument parsing
def to_docopt_arg(p):
    return '<%s>%s' % (mangle_subcommand(p.name), p.get('repeated', False) and '...' or '')


# Return schema' with fields dict: { 'field1' : SchemaField(...), 'SubSchema': schema' }
def to_cli_schema(c, schema):
    res = deepcopy(schema)
    fd = dict()
    res['fields'] = fd

    # util.nested_type_name
    properties = schema.get('properties', dict())
    if not properties and 'variant' in schema and 'map' in schema.variant:
        for e in schema.variant.map:
            assert util.TREF in e
            properties[e.type_value] = e
    # end handle enumerations

    for pn, p in util.items(properties):
        def set_nested_schema(ns):
            if ns.fields:
                fd[pn] = ns
        # end utility

        def dup_property():
            pc = deepcopy(p)
            if 'type' in pc and pc.type == 'string' and 'Count' in pn:
                pc.type = 'int64'
            return pc
        # end

        if util.TREF in p:
            if p[util.TREF] != schema.id: # prevent recursion (in case of self-referential schemas)
                set_nested_schema(to_cli_schema(c, c.schemas[p[util.TREF]]))
        elif p.type == 'array' and 'items' in p and 'type' in p.get('items') and p.get('items').type in POD_TYPES:
            pc = dup_property()
            fd[pn] = SchemaEntry(CTYPE_ARRAY, pc.get('items'), pc)
        elif p.type == 'object':
            if util.is_map_prop(p):
                if 'type' in p.additionalProperties and p.additionalProperties.type in POD_TYPES:
                    pc = dup_property()
                    fd[pn] = SchemaEntry(CTYPE_MAP, pc.additionalProperties, pc)
            else:
                set_nested_schema(to_cli_schema(c, c.schemas[util.nested_type_name(schema.id, pn)]))
        elif p.type in POD_TYPES:
            pc = dup_property()
            fd[pn] = SchemaEntry(CTYPE_POD, pc, pc)
        # end handle property type
    # end

    return res


# Convert the given cli-schema (result from to_cli_schema(schema)) to a yaml-like string. It's suitable for
# documentation only
def cli_schema_to_yaml(schema, prefix=''):
    if not prefix:
        o = '%s%s:\n' % (prefix, schema.id)
    else:
        o = ''
    prefix += '  '
    for fn in sorted(schema.fields.keys()):
        f = schema.fields[fn]
        o += '%s%s:' % (prefix, mangle_subcommand(fn))
        if not isinstance(f, SchemaEntry):
            o += '\n' + cli_schema_to_yaml(f, prefix)
        else:
            t = f.actual_property.type
            if f.container_type == CTYPE_ARRAY:
                t = '[%s]' % t
            elif f.container_type == CTYPE_MAP:
                t = '{ string: %s }' % t
            o += ' %s\n' % t
    # end for each field
    return o


# Return a value string suitable for the given field.
def field_to_value(f):
    v = JSON_TYPE_RND_MAP[f.actual_property.type]()
    if f.container_type == CTYPE_MAP:
        v = 'key=%s' % v
    return v

# split the result along split segments
def process_template_result(r, output_file):
    found = False
    dir = None
    if output_file:
        dir = os.path.dirname(output_file)
        if not os.path.isdir(dir):
            os.makedirs(dir)
    # end handle output directory

    for m in re_splitters.finditer(r):
        found = True
        fh = open(os.path.join(dir, m.group(1)), 'wb')
        fh.write(m.group(2).encode('UTF-8'))
        fh.close()
    # end for each match

    if found:
        r = None

    return r
