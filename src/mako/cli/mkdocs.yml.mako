<%! from util import put_and %>\
<%namespace name="util" file="../lib/util.mako"/>\
site_name: ${util.canonical_name()} v${util.crate_version()}
site_url: ${cargo.doc_base_url}/${util.crate_name()}
site_description: Write integrating applications with bcore

repo_url: ${util.github_source_root_url()}

docs_dir: docs
site_dir: ${mkdocs.site_dir}

pages:
- ['index.md', 'Home']
## - ['be.md', 'Features', 'BE - universal commandline tool']

theme: readthedocs

copyright: Copyright &copy; ${copyright.years}, ${put_and(["`%s`" % a for a in copyright.authors])}

