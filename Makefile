.PHONY:  json-to-xml clean help api-deps regen-apis license update-json cargo
.SUFFIXES:

include Makefile.helpers

VENV := virtualenv
VENV_DIR := .pyenv
PYTHON := $(VENV_DIR)/bin/python
PIP := $(VENV_DIR)/bin/pip
MAKO_RENDER := etc/bin/mako-render
API_VERSION_GEN := etc/bin/api_version_to_yaml.py
TPL := $(PYTHON) $(MAKO_RENDER)

MAKO_SRC = src/mako
RUST_SRC = src/rust
API_DEPS_TPL = $(MAKO_SRC)/deps.mako
API_DEPS = .api.deps
API_SHARED_INFO = etc/api/shared.yaml
API_LIST = etc/api/api-list.yaml
API_JSON_FILES = $(shell find etc -type f -name '*-api.json')
MAKO_LIB_DIR = $(MAKO_SRC)/lib
MAKO_LIB_FILES = $(shell find $(MAKO_LIB_DIR) -type f -name '*.*')

help:
	$(info using template engine: '$(TPL)')
	$(info )
	$(info Targets)
	$(info apis         -    make all APIs)
	$(info cargo        -   run cargo on all APIs, use ARGS="args ..." to specify cargo arguments)
	$(info regen-apis   -   clear out all generated apis, and regenerate them)
	$(info clean-apis   -   delete all generated APIs)
	$(info help-api     -   show all api targets to build individually)
	$(info help         -   print this help)
	$(info license      -   regenerate the main license file)
	$(info update-json  -   copy API definitions from source GOOGLE_GO_APIS_REPO=<path>)
	$(info api-deps     -   generate a file to tell make what API file dependencies will be)

$(PYTHON):
	virtualenv $(VENV_DIR)
	$(PIP) install mako pyyaml

$(MAKO_RENDER): $(PYTHON)

$(API_DEPS): $(API_SHARED_INFO) $(API_DEPS_TPL) $(MAKO_LIB_FILES) $(MAKO_RENDER) $(API_LIST)
	PYTHONPATH=$(MAKO_LIB_DIR) $(TPL) --template-dir '.' -io $(API_DEPS_TPL)=$@ --data-files $(API_SHARED_INFO) $(API_LIST)

api-deps: $(API_DEPS)

$(API_LIST): $(API_VERSION_GEN)
	$(API_VERSION_GEN) etc/api $@ $@

include $(API_DEPS)

LICENSE.md: $(MAKO_SRC)/LICENSE.md.mako $(API_SHARED_INFO)
	PYTHONPATH=$(MAKO_LIB_DIR) $(TPL) -io $<=$@ --data-files $(API_SHARED_INFO)

license: LICENSE.md

regen-apis: clean-apis apis license

clean: clean-apis
	-rm -Rf $(VENV_DIR)
	-rm $(API_DEPS)
	-rm $(API_LIST)

update-json:
	etc/bin/update-json.sh $(GOOGLE_GO_APIS_REPO) etc/api


