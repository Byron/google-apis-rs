.PHONY: help deps regen-apis license test-gen test clean
.SUFFIXES:

VENV_BIN = .virtualenv.marker
VENV_VERSION = 20.24.3

VENV_DIR := .pyenv-$(shell uname)
PYTHON_BIN := $(VENV_DIR)/bin/python

PYTHON := . $(VENV_DIR)/bin/activate; python
PYTEST := $(PYTHON) -m pytest

MAKO_RENDER := etc/bin/mako-render
API_VERSION_GEN := etc/bin/api_version_to_yaml.py
SORT_JSON_FILE := etc/bin/sort_json_file.py
TPL := $(PYTHON) $(MAKO_RENDER)
MKDOCS := $(shell pwd)/$(VENV_DIR)/bin/mkdocs
GHP_IMPORT  := $(shell pwd)/$(VENV_DIR)/bin/ghp-import

GEN_SRC = src/generator
GEN_LIB_SRC = $(GEN_SRC)/lib
MAKO_SRC = src/generator/templates
RUST_SRC = src/rust
PREPROC_DIR = $(RUST_SRC)/preproc
PREPROC = target/release/preproc
API_DEPS_TPL = $(MAKO_SRC)/deps.mako
API_DEPS = .api.deps
CLI_DEPS = .cli.deps
API_DIR = etc/api
API_SHARED_INFO = $(API_DIR)/shared.yaml
TYPE_API_INFO = $(API_DIR)/type-api.yaml
TYPE_CLI_INFO = $(API_DIR)/type-cli.yaml
API_LIST = $(API_DIR)/
ifdef TRAVIS
API_LIST := $(API_LIST)api-list_travis.yaml
else
API_LIST := $(API_LIST)api-list.yaml
endif
API_JSON_FILES = $(shell find etc -type f -name '*-api.json')
MAKO_LIB_FILES = $(shell find $(GEN_LIB_SRC) -type f -name '*.*')
MAKO = export PREPROC=$(PREPROC); export PYTHONPATH=src:$(PYTHONPATH); $(TPL) --template-dir '.'
MAKO_STANDARD_DEPENDENCIES = $(API_SHARED_INFO) $(MAKO_LIB_FILES) $(MAKO_RENDER) $(PREPROC)

help:
	$(info using template engine: '$(MAKO_RENDER)')
	$(info )
	$(info Targets)
	$(info help-api       -   show all api targets to build individually)
	$(info help-cli       -   show all cli targets to build individually)
	$(info docs-all       -   cargo-doc on all APIs and associates, assemble them together and generate index)
	$(info docs-all-clean -   remove the entire set of generated documentation)
	$(info github-pages   -   invoke ghp-import on all documentation)
	$(info regen-apis     -   clear out all generated apis, and regenerate them)
	$(info license        -   regenerate the main license file)
	$(info update-json    -   rediscover API schema json files and update api-list.yaml with latest versions)
	$(info publish-api    -   publish all api crates to crates.io)
	$(info publish-cli    -   publish all cli crates to crates.io, required for `cargo install` to work)
	$(info deps           -   generate a file to tell how to build libraries and programs)
	$(info test-gen       -   run unit tests for python code)
	$(info test           -   run all tests)
	$(info help           -   print this help)

$(PREPROC): $(PREPROC_DIR)/src/main.rs
	cd "$(PREPROC_DIR)" && cargo build --release 

$(VENV_BIN):
	python3 -m pip install --user virtualenv==$(VENV_VERSION)
	touch $@

$(PYTHON_BIN): $(VENV_BIN) requirements.txt
	python3 -m virtualenv -p python3 $(VENV_DIR)
	$@ -m pip install -r requirements.txt

$(MAKO_RENDER): $(PYTHON_BIN) $(wildcard $(GEN_LIB_SRC)/*)

# Explicitly NOT depending on $(MAKO_LIB_FILES), as it's quite stable and now takes 'too long' thanks
# to a URL get call to the google discovery service
$(API_DEPS): $(API_DEPS_TPL) $(API_SHARED_INFO) $(MAKO_RENDER) $(TYPE_API_INFO) $(API_LIST)
	$(MAKO) -io $(API_DEPS_TPL)=$@ --data-files $(API_SHARED_INFO) $(TYPE_API_INFO) $(API_LIST)

$(CLI_DEPS): $(API_DEPS_TPL) $(API_SHARED_INFO) $(MAKO_RENDER) $(TYPE_CLI_INFO) $(API_LIST)
	$(MAKO) -io $(API_DEPS_TPL)=$@ --data-files $(API_SHARED_INFO) $(TYPE_CLI_INFO) $(API_LIST)

deps: $(API_DEPS) $(CLI_DEPS)

include $(API_DEPS)
include $(CLI_DEPS)

LICENSE.md: $(MAKO_SRC)/LICENSE.md.mako $(API_SHARED_INFO) $(MAKO_RENDER)
	$(MAKO) -io $<=$@ --data-files $(API_SHARED_INFO)

license: LICENSE.md

regen-apis: | clean-all-api clean-all-cli gen-all-api gen-all-cli license

test-gen: $(PYTHON_BIN)
	export PYTEST_DISABLE_PLUGIN_AUTOLOAD=1; $(PYTEST) src

test: test-gen

typecheck: $(PYTHON_BIN)
	$(PYTHON) -m pyright $(GEN_LIB_SRC)

clean: clean-all-api clean-all-cli docs-all-clean
	-rm -Rf $(VENV_DIR)
	-rm $(API_DEPS) $(CLI_DEPS)
