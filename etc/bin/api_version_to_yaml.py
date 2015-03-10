#!/usr/bin/env python

# Generate yaml output suitable for use in shared.yaml
# 

import sys
import os
import yaml

isfile = os.path.isfile
isdir = os.path.isdir
join = os.path.join

if __name__ != '__main__':
    raise AssertionError("Not for import")

if len(sys.argv) != 2:
    sys.stderr.write("USAGE: <program> <api_path>, i.e. <program> etc/api\n")
    sys.exit(1)

api_base = sys.argv[1]
if not isdir(api_base):
    raise ValueError("Directory '%s' not accessible" % api_base)

yaml_path = join(api_base, 'shared.yaml')
if not isfile(yaml_path):
    raise AssertionError("Didn't find yaml data at '%s'" % yaml_path)

api_data = list(yaml.load_all(open(yaml_path, 'r')))[0]['api']['list']
for api_name in sorted(os.listdir(api_base)):
    api_path = join(api_base, api_name)
    if not isdir(api_path):
        continue
    last_version = list(sorted(v for v in os.listdir(api_path) if isdir(join(api_path, v))))
    if not last_version:
        continue

    versions = api_data.get('api_name', list())
    if last_version not in versions:
        versions.append(last_version[0])
    api_data[api_name] = list(sorted(versions))
# end for each item in api-base

yaml.dump(dict(api=dict(list=api_data)), sys.stdout, default_flow_style=False)


