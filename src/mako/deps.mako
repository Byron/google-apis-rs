# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !

<%
	import os
	import urllib
	import json

	apis = {}
	api_info = []
	doc_root = directories.output + '/' + directories.doc_subdir
	doc_index = doc_root + '/index.html'

	def to_doc_root(gen_root, crate_name):
		if make.documentation_engine == 'mkdocs':
			return gen_root + '/' + mkdocs.site_dir
		else:
			return gen_root + '/target/doc/' + util.to_extern_crate_name(crate_name)
	# end utility

	central_api_index = lambda crate_name: doc_root + '/' + util.to_extern_crate_name(crate_name) + '/index.html'

	if os.environ.get('FETCH_APIS') is not None:
		discovery_url = 'https://www.googleapis.com/discovery/v1/'
		apis = json.loads(urllib.request.urlopen(discovery_url + "apis").read())

		print('Loaded {} apis from Google'.format(len(apis['items'])))

		for manualy_api in api.get('manually_added', list()):
			apis['items'].append({
				'name': manualy_api['name'],
				'version': manualy_api['version'],
				'discoveryRestUrl': manualy_api['discovery_rest_url']
				})

		print('Total  {} apis'.format(len(apis['items'])))

	json_api_targets = []

	suffix = make.target_suffix
	agsuffix = make.aggregated_target_suffix
	global_targets = make.get('global_targets', False)

	post_processor_arg = ''
	if mako is not UNDEFINED:
		post_processor_arg = '--post-process-python-module=%s' % mako.post_processor_module
%>\
% for an, versions in api.list.items():
% if an in api.get('blacklist', list()):
<% continue %>\
% endif
% for version in versions:
% if an + '-' + version in api.get('blacklist', list()):
<% continue %>\
% endif
<%
	import util
	import os
	import json

	def gen_type_cfg_path(id):
		return '$(API_DIR)/type-' + id + '.yaml'

	CMN_SRC = '/src/client.rs'

	api_name = util.library_name(an, version)
	api_target = util.target_directory_name(an, version, suffix)
	depends_on_target = ''
	if make.depends_on_suffix is not None:
		depends_on_target = directories.output + '/' + util.target_directory_name(an, version, make.depends_on_suffix) + CMN_SRC
	crate_name = util.library_to_crate_name(api_name, suffix)
	gen_root = directories.output + '/' + api_target
	gen_root_stamp = gen_root + '/.timestamp'
	api_common = gen_root + CMN_SRC
	api_clean = api_target + '-clean'
	api_cargo = api_target + '-cargo'
	api_doc = api_target + '-doc'

	api_doc_root = to_doc_root(gen_root, crate_name)
	api_doc_index = api_doc_root + '/index.html'

	# source, destination of individual output files
	sds = [(directories.mako_src + '/' + make.id + '/' + i.source + '.mako', gen_root + '/' +
		   i.get('output_dir', '') + '/' + i.source.strip('../')) for i in make.templates]
	api_json = util.api_json_path(directories.api_base, an, version)
	api_meta_dir = os.path.dirname(api_json)
	print('Loading JSON: {}'.format(api_json))
	try:
		with open(api_json, 'r') as fh:
			crate_version = util.crate_version(cargo.build_version + make.aggregated_target_suffix, json.load(fh).get('revision', '00000000'))
			api_crate_publish_file = api_meta_dir + '/crates/' + crate_version
			api_json_overrides = api_meta_dir + '/' + an + '-api_overrides.yaml'
			type_specific_cfg = gen_type_cfg_path(make.id)
			api_json_inputs = api_json + ' $(API_SHARED_INFO) ' + type_specific_cfg
			if os.path.isfile(api_json_overrides):
				api_json_inputs += ' ' + api_json_overrides
			api_info.append((api_target, api_clean, api_cargo, api_doc, api_crate_publish_file, gen_root))

			space_join = lambda i: ' '.join(a[i] for a in api_info)
	except Exception as e:
		print('Could not open JSON file at {}'.format(api_json))
		print(e)
%>\
${api_common}: $(RUST_SRC)/${make.id}/client.rs $(lastword $(MAKEFILE_LIST)) ${gen_root_stamp}
	@ echo "// COPY OF '$<'"  > $@
	@ echo "// DO NOT EDIT"  >> $@
	@cat $< >> $@

${gen_root_stamp}: $(MAKO_RENDER) ${' '.join(i[0] for i in sds)} ${api_json_inputs} $(MAKO_STANDARD_DEPENDENCIES) ${depends_on_target}
	@echo Generating ${api_target}
	$(MAKO) -io ${' '.join("%s=%s" % (s, d) for s, d in sds)} ${post_processor_arg} --data-files ${api_json_inputs}
	@touch $@

${api_target}: ${api_common}

${api_crate_publish_file}:
	cd ${gen_root} && cargo publish --allow-dirty
	@mkdir -p ${os.path.dirname(api_crate_publish_file)}
	touch $@

${api_cargo}: ${api_target}
	cd ${gen_root} && cargo $(ARGS)

${api_doc_index}: ${api_common}
	% if make.documentation_engine == 'rustdoc':
	cd ${gen_root} && cargo doc
	@echo "Docs for ${api_target} at $@"
	% else:
	@echo mkdocs ${api_doc_index}
	## Our README is the landing page, and thus will serve multiple roles at once !
	@cd ${gen_root} && (mkdir -p ${mkdocs.docs_dir} && cd ${mkdocs.docs_dir} && ln -s ../README.md index.md &>/dev/null) || : && $(MKDOCS) build --clean
	% endif

${api_doc}: ${api_doc_index}

${central_api_index(crate_name)}: ${api_doc_index}
	@test ! -d ${doc_root} && mkdir -p target/doc && ln -s `pwd`/target/doc ${doc_root} || :
	% if make.documentation_engine == 'mkdocs':
	cp -Rf ${api_doc_root} $(dir $@)
	% endif

${api_clean}:
	-rm -Rf ${gen_root}
% endfor
% endfor

clean-all${agsuffix}: ${space_join(1)}
cargo${agsuffix}: ${space_join(2)}
publish${agsuffix}: | gen-all${agsuffix} ${space_join(4)}
gen-all${agsuffix}: ${space_join(0)}

% if global_targets:
${doc_index}: docs-cli ${gen_type_cfg_path('cli')}
	$(MAKO) --var DOC_ROOT=${doc_root} -io $(MAKO_SRC)/index.html.mako=$@ --data-files $(API_SHARED_INFO) $(API_LIST)
	@echo Documentation index created at '$@'
docs-all: ${doc_index}
docs-all-clean:
	rm -Rf ${doc_root}

github-pages: | docs-all-clean docs-all
	ghp-import -n ${doc_root}
	## Have to force-push - allows us to start docs fresh, clearing out unused history
	git push origin +gh-pages

.PHONY += github-pages docs-all docs-all-clean
% endif

docs${agsuffix}: ${' '.join(central_api_index(util.library_to_crate_name(a[0])) for a in api_info)} $(MAKO_STANDARD_DEPENDENCIES)

.PHONY = $(.PHONY) help${agsuffix} clean${agsuffix} cargo${agsuffix} publish${agsuffix} gen-all${agsuffix} ${space_join(0)} ${space_join(1)} ${space_join(2)} ${space_join(3)}

help${agsuffix}:
	$(info gen-all${agsuffix}       -   make all ${make.target_name})
	$(info docs${agsuffix}          -   make all ${make.target_name} documentation)
	$(info clean-all${agsuffix}     -   delete all generated ${make.target_name})
	$(info cargo${agsuffix}         -   run cargo on all ${make.target_name}, use ARGS="args ..." to specify cargo arguments)
	$(info publish${agsuffix}       -   run cargo publish on all ${make.target_name} and remember successful ones with marker files)
% for a in api_info:
	$(info ${a[0]}    -    build the ${a[0]} api)
	$(info ${a[1]}    -    clean all generated files of the ${a[0]} api)
	$(info ${a[2]}    -    run cargo on the ${a[0]} api, using given ARGS="arg1 ...")
	$(info ${a[3]}    -    run cargo doc on the ${a[0]}")
% endfor

% if global_targets:
.PHONY += update-json

% for info in (apis.get('items') or []):
<%
	import util
	import os
	name = util.normalize_library_name(info['name'])
	target = util.api_json_path(directories.api_base, name, info['version'])
	target_dir = os.path.dirname(target)
	## assure the target never actually exists to force him to wget whenver we ask !
	fake_target = target + '-force'
	## Some service urls have $ in them. This may cause the console to treat them as env vars.
	## To handle this properly, we need to escape the $.
	url = info['discoveryRestUrl'].replace("$", "$$")
	json_api_targets.append(fake_target)
%>\
${fake_target}: $(PYTHON_BIN)
	@mkdir -p ${target_dir}
	@-curl --silent --show-error --fail --retry 3 -o '${target}' '${url}'
	$(PYTHON) $(SORT_JSON_FILE) --skip-missing-file '${target}' || rm ${target}
% endfor

update-json: ${' '.join(json_api_targets)}
	$(PYTHON) $(API_VERSION_GEN) $(API_DIR) $(API_LIST) $(API_LIST)
% endif
