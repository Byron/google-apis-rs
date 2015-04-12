import util

import os
import re
import collections

SPLIT_START = '>>>>>>>'
SPLIT_END = '<<<<<<<'

PARAM_FLAG = 'p'
STRUCT_FLAG = 'r'
UPLOAD_FLAG = 'u'
OUTPUT_FLAG = 'o'
VALUE_ARG = 'v'
SCOPE_FLAG = 'scope'

FIELD_SEP = '.'

CONFIG_DIR = '~/.google-service-cli'

re_splitters = re.compile(r"%s ([\w\-\.]+)\n(.*?)\n%s" % (SPLIT_START, SPLIT_END), re.MULTILINE|re.DOTALL)

MethodContext = collections.namedtuple('MethodContext', ['m', 'response_schema', 'params', 'request_value', 
                                                         'media_params' ,'required_props', 'optional_props', 
                                                         'part_prop'])

def new_method_context(resource, method, c):
    m = c.fqan_map[util.to_fqan(c.rtc_map[resource], resource, method)]
    response_schema = util.method_response(c, m)
    params, request_value = util.build_all_params(c, m)
    media_params = util.method_media_params(m)
    required_props, optional_props, part_prop = util.organize_params(params, request_value) 

    return MethodContext(m, response_schema, params, request_value, media_params, 
                         required_props, optional_props, part_prop)


def pretty(n):
    return ' '.join(s.capitalize() for s in mangle_subcommand(n).split('-'))


def is_request_value_property(mc, p):
    return mc.request_value and mc.request_value.id == p.get(util.TREF)

# transform name to be a suitable subcommand
def mangle_subcommand(name):
    return util.camel_to_under(name).replace('_', '-').replace('.', '-')


# transform the resource name into a suitable filename to contain the markdown documentation for it
def subcommand_md_filename(resource, method):
    return mangle_subcommand(resource) + '_' + mangle_subcommand(method) + '.md'


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
        fh.write(m.group(2))
        fh.close()
    # end for each match

    if found:
        r = None

    return r
