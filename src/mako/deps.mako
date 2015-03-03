# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !

<%api_info=[]%>\
% for a in api.list:
<%
	import util
	gen_root = directories.output + '/' + a.name + util.to_api_version(a.version)
	gen_root_stamp = gen_root + '/.timestamp'
	api_name = util.library_name(a.name, a.version)
	api_common = gen_root + '/src/cmn.rs'
	api_clean = api_name + '-clean'
	# source, destination of individual output files
	sds = [(directories.mako_src + '/' + i.source + '.mako', gen_root + '/' + i.get('output_dir', '') + '/' + i.source) 
																								for i in api.templates]
	api_json = directories.api_base + '/' + a.name + '/' + a.version + '/' + a.name + '-api.json'
	api_json_inputs = api_json + " $(API_SHARED_INFO)"
	api_info.append((api_name, api_clean, gen_root))
%>\
${api_common}: $(RUST_SRC)/cmn.rs
	@cp $< $@

${gen_root_stamp}: ${' '.join(i[0] for i in sds)} ${api_json_inputs} $(MAKO_LIB_FILES) $(MAKO_RENDER)
	PYTHONPATH=$(MAKO_LIB_DIR) $(TPL) --template-dir '.' --var OUTPUT_DIR=$@ -io ${' '.join("%s=%s" % (s, d) for s, d in sds)} --data-files ${api_json_inputs}
	@touch $@

${api_name}: ${gen_root_stamp} ${api_common}
	
${api_clean}:
	-rm -Rf ${gen_root}
% endfor

.PHONY += $(.PHONY) help-api clean-apis apis ${' '.join(a[0] for a in api_info)} ${' '.join(a[1] for a in api_info)}

help-api:
	$(info apis       -    make all APIs)
% for a in api_info:
	$(info ${a[0]}    -    build the ${a[0]} api)
	$(info ${a[1]}    -    clean all generated files of the ${a[0]} api)
% endfor

clean-apis: ${' '.join(a[1] for a in api_info)}
apis: ${' '.join(a[0] for a in api_info)}