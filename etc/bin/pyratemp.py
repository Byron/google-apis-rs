#!/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python
# -*- coding: ascii -*-
"""
Commandline-tool for pyratemp.

- check template for syntax-errors
- render templates with data and print the result in utf-8.

Errors and help-messages are written to stderr, resulting data to stdout.

Data can be read as key-value-pairs from the command-line and/or from
JSON- and YAML-files. Additionally, ``date`` and ``mtime_CCMMYYDD``
are set to the current date as "%Y-%m-%d".

By default, HTML-escaping is used for "*.htm" and "*.html" and
LaTeX-escaping for "*.tex".

Exit-codes:

    - 0: ok
    - 1: some Python-modules are missing (import error)
    - 2: --help / usage printed
    - 3: invalid command-line options
    - 10: template syntax-error / parse error
    - 20: datafile error / cannot load data
    - 30: render error

:Version:   0.3.2

:Usage:
    see USAGE or "pyratemp_tool.py --help"

:Requires:  Python >= 2.6 / 3.x, pyratemp, (optional: yaml)

:Author:    Roland Koebler (rk at simple-is-better dot org)
:Copyright: Roland Koebler
:License:   MIT/X11-like, see __license__

:RCS:       $Id: pyratemp_tool.py,v 1.16 2013/09/17 07:45:04 rk Exp $
"""
from __future__ import unicode_literals
from __future__ import print_function

__version__ = "0.3.2"
__author__   = "Roland Koebler <rk at simple-is-better dot org>"
__license__  = """Copyright (c) 2007-2013 by Roland Koebler

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE."""

#-----------------------------------------
USAGE = """pyratemp_tool.py [-s] <-d NAME=VALUE> <-f DATAFILE [-N NAME] [-n NR_OF_ENTRY]> [--xml] TEMPLATEFILES
    -s      syntax-check only (don't render the template)
    -d      define variables (these also override the values from files)
    -f      use variables from a JSON/YAML file
    -n      use nth entry of the JSON/YAML-file
            (JSON: n-th element of the root-array, YAML: n-th entry)
    -N      namespace for variables from the JSON/YAML file
    --xml   encode output as ASCII+xmlcharrefreplace (instead of utf-8)
"""

#-----------------------------------------
import os, sys, getopt, time

try:
    import pyratemp
except ImportError:
    print("ERROR: Python-module 'pyratemp' not found.", file=sys.stderr)
    sys.exit(1)

#-----------------------------------------
def parse(template_name):
    """Parse template + set encoding according to filename-extension.

    :Returns: the parsed template
    """
    ext = os.path.splitext(template_name)[1]
    if   ext == ".htm" or ext == ".html":
        t = pyratemp.Template(filename=template_name, escape=pyratemp.HTML)
    elif ext == ".tex":
        t = pyratemp.Template(filename=template_name, escape=pyratemp.LATEX)
    else:
        t = pyratemp.Template(filename=template_name)
    return t

#----------------------
def load_data(datafiles):
    """Load data from data-files using either 'json' or 'yaml'.

    :Parameters:
        - datafiles: [ [filename, nr_of_entry, namespace], ...]
    :Returns:   read data (dict)
    :Raises:    ImportError, ValueError
    """
    imported_json = False
    imported_yaml = False
    mydata = {}

    for filename, n, namespace in datafiles:
        if filename[-5:].lower() == ".json":
            if not imported_json:
                try:
                    import simplejson as json
                except ImportError:
                    import json
                imported_json = True
            try:
                myjson = json.load(open(filename, 'r'))
                if n != -1:
                    myjson = myjson[n]

                if namespace is None:
                    mydata.update(myjson)
                else:
                    mydata.update({namespace: myjson})
            except ValueError as err:
                raise ValueError("Invalid JSON in file '%s'. (%s)" % (filename, str(err)))
        elif filename[-5:].lower() == ".yaml":
            if not imported_yaml:
                import yaml
                imported_yaml = True
            if n == -1:
                n = 0
            myyaml = yaml.load_all(open(filename, 'r'))
            if namespace is not None:
                mydata.update({namespace: list(myyaml)[n]})
            else:
                mydata.update(list(myyaml)[n])
        else:
            raise ValueError("Invalid data-file '%s', must be .json or .yaml" % filename)
    return mydata

#-----------------------------------------
if __name__ == "__main__":
    # parse parameters
    try:
        opt_list, files = getopt.getopt(sys.argv[1:], "sd:f:n:N:h", ("help", "xml"))
    except getopt.GetoptError as err:
        print("ERROR: Invalid option. (%s)" % err, file=sys.stderr)
        sys.exit(3)
    render = True
    template_name = ""
    namevals = {}
    datafiles = []      #[ [filename, nr_of_entry], ...]
    output_xml = False
    for key, value in opt_list:
        if "-h" == key or "--help" == key:
            print(USAGE, file=sys.stderr)
            sys.exit(2)
        elif "-s" == key:
            render = False
        elif "-d" == key:
            (name, value) = value.split("=", 1)
            namevals[name] = value
        elif "-f" == key:
            datafiles.append([value, -1, None])
        elif "-n" == key:
            if not datafiles:
                print("ERROR: -n only allowed after -f.", file=sys.stderr)
                sys.exit(3)
            datafiles[-1][1] = int(value)
        elif "-N" == key:
            if not datafiles:
                print("ERROR: -N only allowed after -f.", file=sys.stderr)
                sys.exit(3)
            datafiles[-1][2] = value
        elif "--xml" in key:
            output_xml = True
    if not files:
        print(USAGE, file=sys.stderr)
        sys.exit(2)
    for f in files:
        if f == "--":
            break
        elif f[0] == "-":
            print("ERROR: Invalid order of parameters. (%s)" % f, file=sys.stderr)
            sys.exit(3)

    # template
    for template_name in files:
        # parse + syntax-check
        try:
            t = parse(template_name)
        except pyratemp.TemplateSyntaxError as err:
            print("file '%s':" % template_name, file=sys.stderr)
            print("  TemplateSyntaxError:", str(err), file=sys.stderr)
            sys.exit(10)

        if render:
            # load data
            try:
                filedata = load_data(datafiles)
            except ImportError as err:
                print("ImportError/missing Python-module:", str(err), file=sys.stderr)
                sys.exit(1)
            except ValueError as err:
                print("Datafile error:", str(err), file=sys.stderr)
                sys.exit(20)

            localtime = time.localtime()
            data = {
                    'mtime_CCYYMMDD':time.strftime("%Y-%m-%d",localtime),
                    'date'          :time.strftime("%Y-%m-%d",localtime),
                    }
            data.update(filedata)
            data.update(namevals)
            data = pyratemp.dictkeyclean(data)

            # render
            try:
                if output_xml:
                    result = t(**data).encode("ascii", "xmlcharrefreplace")
                else:
                    result = t(**data).encode("utf-8")
                os.write(sys.stdout.fileno(), result)
            except pyratemp.TemplateRenderError as err:
                print("file '%s':\n" % template_name, file=sys.stderr)
                print("  TemplateRenderError:", str(err), file=sys.stderr)
                sys.exit(30)

#-----------------------------------------

