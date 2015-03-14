# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !

<%
	import os
	import urllib2
	import json

	api_info = []
	doc_root = directories.output + '/doc'
	doc_index = doc_root + '/index.html'

	to_doc_root = lambda gen_root, api_name: gen_root + '/target/doc/' + api_name
	central_api_index = lambda api_name: doc_root + '/' + api_name + '/index.html'

	discovery_url = 'https://www.googleapis.com/discovery/v1/'
	apis = json.loads(urllib2.urlopen(discovery_url + "apis").read())
	json_api_targets = []
%>\
% for an, versions in api.list.iteritems():
% if an in api.get('blacklist', list()):
<% continue %>\
% endif
% for version in versions:
<%
	import util
	import os
	api_name = util.library_name(an, version)
	gen_root = directories.output + '/' + api_name
	gen_root_stamp = gen_root + '/.timestamp'
	api_common = gen_root + '/src/cmn.rs'
	api_clean = api_name + '-clean'
	api_cargo = api_name + '-cargo'
	api_doc = api_name + '-doc'

	api_doc_root = to_doc_root(gen_root, api_name)
	api_doc_index = api_doc_root + '/index.html'

	# source, destination of individual output files
	sds = [(directories.mako_src + '/' + i.source + '.mako', gen_root + '/' + i.get('output_dir', '') + '/' + i.source) 
																								for i in api.templates]
	api_json = directories.api_base + '/' + an + '/' + version + '/' + an + '-api.json'
	api_json_overrides = os.path.dirname(api_json) + '/' + an + '-api_overrides.json'
	api_json_inputs = api_json + ' $(API_SHARED_INFO)'
	if os.path.isfile(api_json_overrides):
		api_json_inputs += ' ' + api_json_overrides
	api_info.append((api_name, api_clean, api_cargo, api_doc, gen_root))

	space_join = lambda i: ' '.join(a[i] for a in api_info)
%>\
${api_common}: $(RUST_SRC)/cmn.rs $(lastword $(MAKEFILE_LIST)) ${gen_root_stamp}
	@ echo "// COPY OF '$<'"  > $@
	@ echo "// DO NOT EDIT"  >> $@
	@cat $< >> $@

${gen_root_stamp}: ${' '.join(i[0] for i in sds)} ${api_json_inputs} $(MAKO_STANDARD_DEPENDENCIES)
	@echo Generating ${api_name}
	@$(MAKO) --template-dir '.' --var OUTPUT_DIR=$@ -io ${' '.join("%s=%s" % (s, d) for s, d in sds)} --data-files ${api_json_inputs}
	@touch $@

${api_name}: ${api_common}

${api_cargo}: ${api_name}
	(cd ${gen_root} && cargo $(ARGS))

${api_doc_index}: ${api_name}
	(cd ${gen_root} && cargo doc)
	@echo "Docs for ${api_name} at $@"

${api_doc}: ${api_doc_index}

${central_api_index(api_name)}: ${api_doc_index}
	@mkdir -p ${doc_root}
	cp -Rf ${os.path.dirname(to_doc_root(gen_root, api_name))}/* ${doc_root}

${api_clean}:
	-rm -Rf ${gen_root}
% endfor
% endfor

clean-apis: ${space_join(1)} docs-clean
cargo: ${space_join(2)}
apis: ${space_join(0)}

${doc_index}: ${' '.join(central_api_index(a[0]) for a in api_info)} $(MAKO_STANDARD_DEPENDENCIES)
	$(MAKO) --var DOC_ROOT=${doc_root} -io $(MAKO_SRC)/index.html.mako=$@ --data-files $(API_SHARED_INFO) $(API_LIST)
	@echo Documentation index created at '$@'

docs: ${doc_index}
docs-clean:
	rm -Rf ${doc_root}  

github-pages: | docs-clean docs 
	ghp-import -n ${doc_root}
	## Have to force-push - I think it resets the branch, thus not keeping history (?)
	git push origin +gh-pages

.PHONY = $(.PHONY) update-json github-pages help-api clean-apis cargo apis docs docs-clean ${space_join(0)} ${space_join(1)} ${space_join(2)} ${space_join(3)}

help-api:
	$(info apis       -    make all APIs)
% for a in api_info:
	$(info ${a[0]}    -    build the ${a[0]} api)
	$(info ${a[1]}    -    clean all generated files of the ${a[0]} api)
	$(info ${a[2]}    -    run cargo on the ${a[0]} api, using given ARGS="arg1 ...")
	$(info ${a[3]}    -    run cargo doc on the ${a[0]}")
% endfor

% for info in apis['items']:
<%
	target_dir = directories.api_base + '/' + info['name'] + '/' + info['version']
	target = target_dir + '/' + info['name'] + '-api.json'
	## assure the target never actually exists to force him to wget whenver we ask !
	fake_target = target + '-force'
	json_api_targets.append(fake_target)
%>\
${fake_target}:
	@mkdir -p ${target_dir}
	@wget -nv ${discovery_url + info['discoveryLink']} -O ${target}
% endfor

update-json: ${' '.join(json_api_targets)}
	$(API_VERSION_GEN) etc/api $(API_LIST) $(API_LIST)
