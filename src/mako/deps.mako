# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !

<%api_info=[]%>\
% for an, versions in api.list.iteritems():
% if an in api.blacklist:
<% continue %>\
% endif
% for version in versions:
<%
	import util
	import os
	gen_root = directories.output + '/' + an + util.to_api_version(version)
	gen_root_stamp = gen_root + '/.timestamp'
	api_name = util.library_name(an, version)
	api_common = gen_root + '/src/cmn.rs'
	api_clean = api_name + '-clean'
	api_cargo = api_name + '-cargo'
	# source, destination of individual output files
	sds = [(directories.mako_src + '/' + i.source + '.mako', gen_root + '/' + i.get('output_dir', '') + '/' + i.source) 
																								for i in api.templates]
	api_json = directories.api_base + '/' + an + '/' + version + '/' + an + '-api.json'
	api_json_overrides = os.path.dirname(api_json) + '/' + an + '-api_overrides.json'
	api_json_inputs = api_json + " $(API_SHARED_INFO)"
	if os.path.isfile(api_json_overrides):
		api_json_inputs += ' ' + api_json_overrides
	api_info.append((api_name, api_clean, api_cargo, gen_root))

	space_join = lambda i: ' '.join(a[i] for a in api_info)
%>\
${api_common}: $(RUST_SRC)/cmn.rs $(lastword $(MAKEFILE_LIST)) ${gen_root_stamp}
	@ echo "// COPY OF '$<'"  > $@
	@ echo "// DO NOT EDIT"  >> $@
	@cat $< >> $@

${gen_root_stamp}: ${' '.join(i[0] for i in sds)} ${api_json_inputs} $(MAKO_LIB_FILES) $(MAKO_RENDER)
	@echo Generating ${api_name}
	@PYTHONPATH=$(MAKO_LIB_DIR) $(TPL) --template-dir '.' --var OUTPUT_DIR=$@ -io ${' '.join("%s=%s" % (s, d) for s, d in sds)} --data-files ${api_json_inputs}
	@touch $@

${api_name}: ${gen_root_stamp} ${api_common}

${api_cargo}: ${api_name}
	cd ${gen_root} && cargo $(ARGS)

${api_clean}:
	-rm -Rf ${gen_root}
% endfor
% endfor

.PHONY += $(.PHONY) help-api clean-apis apis ${space_join(0)} ${space_join(1)} ${space_join(2)}

help-api:
	$(info apis       -    make all APIs)
% for a in api_info:
	$(info ${a[0]}    -    build the ${a[0]} api)
	$(info ${a[1]}    -    clean all generated files of the ${a[0]} api)
	$(info ${a[2]}    -    run cargo on the ${a[0]} api, using given ARGS="arg1 ...")
% endfor

clean-apis: ${space_join(1)}
cargo: ${space_join(2)}
apis: ${space_join(0)}