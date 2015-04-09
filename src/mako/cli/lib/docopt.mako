<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from cli import mangle_subcommand
%>\
<%def name="new(c)">\
docopt!(Args derive Debug, "
Usage: 
    % for resource in sorted(c.rta_map.keys()):
<%
        mangled_method_names = [mangle_subcommand(method) for method in sorted(c.rta_map[resource])]
        assert mangled_method_names
%>\
    ${util.program_name()} ${mangle_subcommand(resource)} \
% if len(mangled_method_names) > 1:
(\
% endif
${'|'.join(mangled_method_names)}\
% if len(mangled_method_names) > 1:
)\
% endif

    % endfor # end for each resource
    ${util.program_name()} --help
");
</%def>