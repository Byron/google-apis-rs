.PHONY:  json-to-xml clean help api-deps

include Makefile.helpers

PYTHON = python2.7
TPL = etc/bin/pyratemp.py

API_DEPS = .api.deps
API_SHARED_INFO = ./etc/api/shared.yml
API_JSON_FILES = $(shell find ./etc -type f -name '*-api.json')

help:
	$(info Programs)
	$(info ----> GSL: '$(GSL)')
	$(info ----> templat engine: '$(TPL)')
	$(info )
	$(info Targets)
	$(info help         -   print this help)
	$(info api-deps     -   generate a file to tell make what API file dependencies will be)

json-to-xml: $(API_XML_FILES)
$(API_DEPS): $(API_SHARED_INFO)
	$(TPL) -f $(API_SHARED_INFO) -d DEP_FILE=$@

api-deps: $(API_DEPS)

clean:
	-rm $(API_DEPS)
	-rm $(API_XML_FILES)


