# DO NOT EDIT !
# This file was generated automatically by '${self.uri}'
# DO NOT EDIT !

<%api_info=[]%>\
% for a in api.list:
<%
	gen_root = directories.output + '/' + a.name + a.version[1:]
	api_name = a.name + a.version
	api_clean = api_name + '-clean'
	# source, destination
	sds = [(directories.mako_src + '/' + i.source + '.mako', gen_root + i.get('output_dir', '') + '/' + i.source) 
																								for i in api.templates]
	api_json = directories.api_base + '/' + a.name + '/' + a.version + '/' + a.name + '-api.json'
	api_json_inputs = api_json + " $(API_SHARED_INFO)"
	api_info.append((api_name, api_clean, gen_root))
%>\
${gen_root}: ${' '.join(i[0] for i in sds)} ${api_json_inputs}
	@mkdir -p $@
	$(TPL) --var OUTPUT_DIR=$@ -io ${' '.join("%s=%s" % (s, d) for s, d in sds)} --data-files ${api_json_inputs}

${api_name}: ${gen_root}
	
${api_clean}:
	-rm -Rf ${gen_root}
% endfor

.PHONY += $(.PHONY) ${' '.join(a[0] for a in api_info)} ${' '.join(a[1] for a in api_info)}

help-api:
% for a in api_info:
	$(info ${a[0]}    -    build the ${a[0]} api)
	$(info ${a[1]}    -    clean all generated files of the ${a[0]} api)
% endfor

clean-api: ${' '.join(a[1] for a in api_info)}