import util

import os
import re

SPLIT_START = '>>>>>>>'
SPLIT_END = '<<<<<<<'

re_splitters = re.compile(r"%s ([\w\-\.]+)\n(.*?)\n%s" % (SPLIT_START, SPLIT_END), re.MULTILINE|re.DOTALL)

# transform name to be a suitable subcommand
def mangle_subcommand(name):
    return util.camel_to_under(util.singular(name)).replace('_', '-').replace('.', '-')


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
