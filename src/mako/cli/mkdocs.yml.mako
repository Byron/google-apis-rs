<%
    from util import (put_and, new_context)
    from cli import (subcommand_md_filename, mangle_subcommand)

    c = new_context(schemas, resources, context.get('methods'))
%>\
<%namespace name="util" file="../lib/util.mako"/>\
site_name: ${util.canonical_name()} v${util.crate_version()}
site_url: ${cargo.doc_base_url}/${util.crate_name()}
site_description: Write integrating applications with bcore

repo_url: ${util.github_source_root_url()}

docs_dir: ${mkdocs.docs_dir}
site_dir: ${mkdocs.site_dir}

pages:
- ['index.md', 'Home']
% for resource in sorted(c.rta_map.keys()):
- ['${subcommand_md_filename(resource)}', 'Commands', '${' '.join(s.capitalize() for s in mangle_subcommand(resource).split('-'))}']
% endfor

theme: readthedocs

copyright: Copyright &copy; ${copyright.years}, ${put_and(["`%s`" % a for a in copyright.authors])}

