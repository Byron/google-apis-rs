<%api_info=[]%>
% for a in api.list:
<%
	gen_root = directories.output + '/' + a.name + '_' + a.version
	api_name = a.name + a.version
	api_clean = api_name + '-clean'
	api_info.append((api_name, api_clean, gen_root))
%>
${gen_root}: ${directories.api_base}/${a.name}/${a.version}/${a.name}-api.json ${SHARED_INFO_FILE}
${api_name}: ${gen_root}
	@echo TODO ${api_name} command
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