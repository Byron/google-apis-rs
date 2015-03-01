.PHONY:  json-to-xml clean help api-deps

include Makefile.helpers

PYTHON = python2.7
CONVERT = $(PYTHON) ./etc/bin/json2xml.py
GSL = ./etc/bin/gsl_$(OS)-$(ARCH) -q

API_DEPS = .api.deps
API_SHARED_XML = ./etc/api/shared.xml
API_JSON_FILES = $(shell find ./etc -type f -name '*-api.json')
API_XML_FILES = $(patsubst %.json,%.xml,$(API_JSON_FILES))

help:
	$(info Programs)
	$(info ----> GSL: '$(GSL)')
	$(info ----> json2xml: '$(CONVERT)')
	$(info )
	$(info Targets)
	$(info help         -   print this help)
	$(info json-to-xml  -   convert json API files to xml for consumption by GSL)
	$(info api-deps     -   generate a file to tell make what API file dependencies will be)

json-to-xml: $(API_XML_FILES)
$(API_DEPS): $(API_XML_FILES)
	$(GSL) -script:src/gsl/deps.gsl $(API_SHARED_XML)

%.xml: %.json
	$(CONVERT) --pretty -o $@ < $<

api-deps: $(API_DEPS)

clean:
	-rm $(API_DEPS)
	-rm $(API_XML_FILES)


