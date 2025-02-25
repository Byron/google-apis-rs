#!/usr/bin/env python

# mako/cmd.py
# Copyright (C) 2006-2015 the Mako authors and contributors <see AUTHORS file>
#
# This module is part of Mako and is released under
# the MIT License: http://www.opensource.org/licenses/mit-license.php
import json
import os
import sys
from argparse import ArgumentParser
from copy import deepcopy
from importlib import import_module
from os.path import dirname, isfile

import yaml
from mako import exceptions
from mako.lookup import TemplateLookup
from mako.template import Template


# From https://github.com/Byron/bcore
class DictObject(object):
    """An object which wraps a dictionary to allow object.key access.
    If the source dictionary doesn't contain any sub-dictionaries, the input
    dict will be referenced. Otherwise it will be copied.

    An attribute error is raised if a value is not accessible.

    Please note that you cannot access dict keys which are not valid attribute names.
    """

    _default_dict = dict()
    _unpackable_types = (dict, tuple, list)

    def __init__(self, indict=_default_dict):
        """Initialize this instance from an input dictionary. If it contains other dictionaries, those will
        trigger their parent dictionaries to be copied, as they will be used as DictObject themselves and
        placed in the copy accordingly.
        NOTE: other DictObjects are used by reference. Generally, this type tries to perform the least
        amount of copying possible."""
        if indict is self._default_dict:
            return
        # end handle default instantiation, which makes us empty
        if isinstance(indict, DictObject):
            self.__dict__ = indict.__dict__
            return
        # END handle special case, be a reference
        dct = indict
        for key, val in dct.items():
            if isinstance(val, self._unpackable_types):
                dct = None
                break
        # END for each key-value pair

        if dct is None:
            dct = dict(indict)

            def unpack(val):
                """unpack helper"""
                if isinstance(val, dict):
                    val = DictObject(val)
                elif isinstance(val, (tuple, list)):
                    val = type(val)(unpack(item) for item in val)
                return val

            # END unpack
            for key, val in dct.items():
                dct[key] = unpack(val)
            # END for each k,v pair
        # END handle recursive copy
        self.__dict__ = dct

    def __str__(self):
        return str(self.__dict__)

    def __repr__(self):
        return repr(self.__dict__)

    def __getitem__(self, name):
        try:
            return getattr(self, name)
        except AttributeError:
            raise KeyError(name)
        # end convert exception

    def __setitem__(self, name, value):
        setattr(self, name, value)

    def __contains__(self, name):
        return name in self.__dict__

    def __len__(self):
        return len(self.__dict__)

    def __iter__(self):
        return iter(self.__dict__)

    def __eq__(self, other):
        """Compares a possibly expensive comparison"""
        if isinstance(other, DictObject):
            # EXPENSIVE !
            return self.to_dict() == other.to_dict()
        elif isinstance(other, dict):
            return self.to_dict() == other
        # end handle type of other
        return self is other

    def update(self, other, **kwargs):
        """Similar to dict.update"""
        items = other
        if hasattr(other, "keys"):
            items = other.items()
        for item_list in (items, kwargs.items()):
            for k, v in item_list:
                setattr(self, k, v)
        # end for each item list

    def to_dict(self, recursive=False):
        """@return ourselves as normal dict
        @param recursive if True, a recursive copy will be returned if required."""
        if recursive:

            def obtain_needs_copy(value):
                """figure out if a copy is required"""
                if isinstance(value, DictObject):
                    return True
                if isinstance(value, (tuple, list, set)):
                    for item in value:
                        if obtain_needs_copy(item):
                            return True
                        # end check needs copy
                    # end for each item in value
                # end if instance is iterable
                return False

            # end check needs copy

            def unpack(val):
                """unpack val recursively and copy it gently"""
                if isinstance(val, DictObject):
                    val = val.to_dict(recursive)
                elif isinstance(val, (tuple, list, set)):
                    val = type(val)(unpack(item) for item in val)
                # end handle type resolution
                return val

            # end unpack

            needs_copy = False
            for value in self.__dict__.values():
                if obtain_needs_copy(value):
                    needs_copy = True
                    break
                # end check value
            # END for each value

            if needs_copy:
                new_dict = dict()
                for key, val in self.__dict__.items():
                    new_dict[key] = unpack(val)
                # END for each key, value pair
                return new_dict
            # else:
            #   just fall through and return ourselves as dictionary

        # END handle recursion
        return self.__dict__

    def copy(self):
        """@return a (deep) copy of self"""
        return type(self)(self.to_dict())

    def clone(self):
        """@return a deep copy of this dict. This onyl means that the key-sets are independent. However, the
        values are still shared, which matters in case of lists for instance"""
        return type(self)(deepcopy(self.to_dict(recursive=True)))

    def inversed_dict(self):
        """@return new dictionary which uses this dicts keys as values, and values as keys
        @note duplicate values will result in just a single key, effectively drupping items.
        Use this only if you have unique key-value pairs"""
        return dict(list(zip(list(self.__dict__.values()), list(self.__dict__.keys()))))

    def get(self, name, default=None):
        """as dict.get"""
        return self.__dict__.get(name, default)

    def keys(self):
        """as dict.keys"""
        return list(self.__dict__.keys())

    def values(self):
        """as dict.values"""
        return list(self.__dict__.values())

    def items(self):
        """as dict.items"""
        return list(self.__dict__.items())

    def _items(self):
        """as dict.items, avoiding name clashes"""
        return list(self.__dict__.items())

    def pop(self, key, default=sys):
        """as dict.pop"""
        if default is sys:
            return self.__dict__.pop(key)
        else:
            return self.__dict__.pop(key, default)
        # end assure semantics are kept


# end class DictObject


def load_data(datafiles):
    """Load data from data-files using either 'json' or 'ya?ml'.

    :Parameters:
        - datafiles: [ [filename, namespace], ...]
    :Returns:   data (dict)
    :Raises:    ImportError, ValueError
    """
    imported_yaml = False
    mydata = {}

    for filename, namespace in datafiles:
        data = None
        if filename[-5:].lower() == ".json":
            try:
                data = json.load(open(filename, "r"))
            except ValueError as err:
                raise ValueError(
                    "Invalid JSON in file '%s'. (%s)" % (filename, str(err))
                )
        elif filename[-5:].lower() in (".yaml", ".yml"):
            data = yaml.load_all(open(filename, "r"), Loader=yaml.loader.FullLoader)
            data = list(data)[0]
        else:
            raise ValueError(
                "Invalid data-file '%s', must be .json, .yaml or .yml" % filename
            )
        assert data is not None
        if namespace:
            data = {namespace: data}
        mydata = merge(mydata, data)
    return mydata


def varsplit(var):
    if "=" not in var:
        return (var, "")
    return var.split("=", 1)


def _exit():
    sys.stderr.write(exceptions.text_error_template().render())
    sys.exit(1)


def cmdline(argv=None):

    parser = ArgumentParser("mako-render")
    parser.add_argument(
        "--var",
        nargs="*",
        default=[],
        help="variable (can be used multiple times, use NAME=VALUE)",
    )
    parser.add_argument(
        "--data-files",
        nargs="*",
        default=[],
        help="data file (can be used multiple times, use path[=namespace])",
    )
    parser.add_argument(
        "--template-dir",
        nargs="*",
        default=[],
        help="Directory to use for template lookup (multiple "
        "directories may be provided). If not given then if the "
        "template is read from stdin, the value defaults to be "
        "the current directory, otherwise it defaults to be the "
        "parent directory of the file provided.",
    )
    parser.add_argument(
        "-io",
        nargs="+",
        help="input and ouptut pairs. can be used multiple times, use TEMPLATE_FILE_IN=[OUTPUT_FILE])",
    )
    parser.add_argument(
        "--post-process-python-module",
        default="",
        help="Specify a python module with a `module.process_template_result(r, output_file|None) -> None|r'."
        "If it returns None, no output file will be written. Use it to perform any operation on "
        "the template's result. The module, like 'foo.handler' will be imported and thus "
        " needs to be in the PYTHONPATH.",
    )

    options = parser.parse_args(argv)
    if len(options.io) == 0:
        options.io.append("-")
    options.io = [varsplit(v) for v in options.io]

    datafiles = [varsplit(var) for var in options.data_files]
    data = load_data(datafiles)
    data_converted = dict(
        (k, DictObject(v)) for k, v in data.items() if isinstance(v, dict)
    )
    data_converted.update((k, v) for k, v in data.items() if not isinstance(v, dict))
    data_converted.update(dict([varsplit(var) for var in options.var]))
    del data

    post_processor = lambda r, of: r
    if options.post_process_python_module:
        fn_name = "process_template_result"
        pm = import_module(options.post_process_python_module)
        post_processor = getattr(pm, fn_name, None)
        if post_processor is None:
            raise AssertionError(
                "python module '%s' must have a function called '%s'"
                % (options.post_process_python_module, fn_name)
            )
    # end handle post processor

    seen_stdin = False
    for input_file, output_file in options.io:
        if input_file == "-":
            assert not seen_stdin, "STDIN (-) can only be named once"
            seen_stdin = True
            lookup_dirs = options.template_dir or ["."]
            lookup = TemplateLookup(lookup_dirs)
            try:
                template = Template(sys.stdin.read(), lookup=lookup)
            except:
                _exit()
        else:
            if not isfile(input_file):
                raise SystemExit("error: can't find %s" % input_file)
            lookup_dirs = options.template_dir or [dirname(input_file)]
            lookup = TemplateLookup(lookup_dirs)
            try:
                template = Template(filename=input_file, lookup=lookup)
            except:
                _exit()

        try:
            result = post_processor(template.render(**data_converted), output_file)
            if result is None:
                continue
            if output_file:
                dir = dirname(output_file)
                if dir and not os.path.isdir(dir):
                    os.makedirs(dir)
                fh = open(output_file, "wb")
                fh.write(result.encode("utf-8"))
                fh.close()
            else:
                print(result.encode("utf-8"))
        except:
            _exit()
    # end for each input file


# From http://stackoverflow.com/questions/7204805/dictionaries-of-dictionaries-merge
# But: We overwrite leafs unconditionally
def merge(a, b, path=None):
    if path is None:
        path = []
    for key in b:
        if key in a:
            if isinstance(a[key], dict) and isinstance(b[key], dict):
                merge(a[key], b[key], path + [str(key)])
            elif b[key] is None:
                del a[key]
            else:
                # overwrite leafs unconditionally !
                if isinstance(a[key], list) and isinstance(b[key], list):
                    a[key] = a[key] + b[key]
                else:
                    a[key] = b[key]
        else:
            a[key] = b[key]
    return a


if __name__ == "__main__":
    cmdline()
