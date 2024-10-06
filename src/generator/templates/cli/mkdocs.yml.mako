<%
    from generator.lib.util import (put_and, new_context)
    from generator.lib.cli import (subcommand_md_filename, mangle_subcommand, pretty)

    c = new_context(schemas, resources)
%>\
<%namespace name="util" file="../../lib/util.mako"/>\
site_name: ${util.canonical_name()} v${util.crate_version()}
site_url: ${cargo.doc_base_url}/${util.crate_name()}
site_description: A complete library to interact with ${util.canonical_name()} (protocol ${version})

repo_url: ${util.github_source_root_url()}

docs_dir: ${mkdocs.docs_dir}
site_dir: ${mkdocs.site_dir}

nav:
- Home: 'index.md'
% for resource in sorted(c.rta_map.keys()):
- '${pretty(resource)}':
% for method in sorted(c.rta_map[resource]):
    - '${pretty(method)}': '${subcommand_md_filename(resource, method)}'
% endfor # each method
% endfor # each resource

theme: readthedocs

copyright: Copyright &copy; ${copyright.years}, ${put_and(["`%s`" % a for a in copyright.authors])}
