.PHONY:  json-to-xml clean help api-deps

include Makefile.helpers

PYTHON = python2.7
CONVERT = $(PYTHON) ./etc/bin/json2xml.py
GSL = ./etc/bin/gsl_$(OS)-$(ARCH)

API_DEPS = .api.deps
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

%.xml: %.json
	$(CONVERT) --pretty -o $@ < $<

clean:
	-rm $(API_XML_FILES)


