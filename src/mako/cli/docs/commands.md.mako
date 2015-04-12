<%namespace name="util" file="../../lib/util.mako"/>\
<%
    from util import (hash_comment, new_context, method_default_scope)
    from cli import (subcommand_md_filename, new_method_context, SPLIT_START, SPLIT_END, pretty, SCOPE_FLAG,
                     mangle_subcommand)

    c = new_context(schemas, resources, context.get('methods'))
%>\
% for resource in sorted(c.rta_map.keys()):
% for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)
%>\
${SPLIT_START} ${subcommand_md_filename(resource, method)}
# ${pretty(resource)}: ${pretty(method)}
% if mc.m.description:
${mc.m.description}
% endif # show method description
% if mc.m.get('scopes'):
# Scopes

You will need authorization for \
% if len(mc.m.scopes) > 1:
at least one of the following scopes to make a valid call:

% for s in mc.m.scopes:
* *${s}*
% endfor
% else:
the *${mc.m.scopes[0]}* scope to make a valid call.
% endif # len(scopes) > 1

If unset, the scope for this method defaults to *${method_default_scope(mc.m)}*.
You can set the scope for this method like this: `${util.program_name()} --${SCOPE_FLAG} <scope> ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ...`
% endif # have method scopes
${SPLIT_END}
% endfor # each method
% endfor # each resource
