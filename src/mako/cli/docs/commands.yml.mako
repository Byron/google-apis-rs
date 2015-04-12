<%namespace name="util" file="../../lib/util.mako"/>\
<%
    from util import (hash_comment, new_context)
    from cli import (subcommand_md_filename, SPLIT_START, SPLIT_END)

    c = new_context(schemas, resources, context.get('methods'))
%>\
% for resource in sorted(c.rta_map.keys()):
% for method in sorted(c.rta_map[resource]):
${SPLIT_START} ${subcommand_md_filename(resource, method)}
${resource.upper()} DOCS

${method.upper()}
${SPLIT_END}
% endfor # each method
% endfor # each resource