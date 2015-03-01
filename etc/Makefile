.PHONY: json-to-xml clean

CONVERT = ./json2xml.py

API_JSON_FILES = $(shell find . -type f -name '*-api.json')
API_XML_FILES = $(patsubst %.json,%.xml,$(API_JSON_FILES))

help:
	$(info Valid targets are:)
	$(info 	json-to-xml   -   convert json API files to xml for consumption by GSL)

json-to-xml: $(API_XML_FILES)

%.xml: %.json
	$(CONVERT) --pretty -o $@ < $<

clean:
	-rm $(API_XML_FILES)

