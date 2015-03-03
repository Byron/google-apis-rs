.PHONY:  json-to-xml clean help api-deps regen-apis license
.SUFFIXES:

include Makefile.helpers

VENV := virtualenv
VENV_DIR := .pyenv
PYTHON := $(VENV_DIR)/bin/python
PIP := $(VENV_DIR)/bin/pip
MAKO_RENDER := etc/bin/mako-render
TPL := $(PYTHON) $(MAKO_RENDER)

MAKO_SRC = src/mako
RUST_SRC = src/rust
API_DEPS_TPL = $(MAKO_SRC)/deps.mako
API_DEPS = .api.deps
API_SHARED_INFO = etc/api/shared.yaml
API_JSON_FILES = $(shell find etc -type f -name '*-api.json')
MAKO_LIB_DIR = $(MAKO_SRC)/lib
MAKO_LIB_FILES = $(shell find $(MAKO_LIB_DIR) -type f -name '*.*')

help:
	$(info using template engine: '$(TPL)')
	$(info )
	$(info Targets)
	$(info help         -   print this help)
	$(info api-deps     -   generate a file to tell make what API file dependencies will be)
	$(info regen-apis   -   clear out all generated apis, and regenerate them)
	$(info help-api     -   show all api targets to build individually)
	$(info license      -   regenerate the main license file)

$(PYTHON):
	virtualenv $(VENV_DIR)
	$(PIP) install mako pyyaml

$(MAKO_RENDER): $(PYTHON)

$(API_DEPS): $(API_SHARED_INFO) $(API_DEPS_TPL) $(MAKO_LIB_FILES) $(MAKO_RENDER)
	PYTHONPATH=$(MAKO_LIB_DIR) $(TPL) --template-dir '.' -io $(API_DEPS_TPL) --data-files $(API_SHARED_INFO) > $@

api-deps: $(API_DEPS)

include $(API_DEPS)

LICENSE.md: $(MAKO_SRC)/LICENSE.md.mako $(API_SHARED_INFO)
	PYTHONPATH=$(MAKO_LIB_DIR) $(TPL) -io $<=$@ --data-files $(API_SHARED_INFO)

license: LICENSE.md

regen-apis: clean-apis apis license

clean: clean-apis
	-rm -Rf $(VENV_DIR)
	-rm $(API_DEPS)


