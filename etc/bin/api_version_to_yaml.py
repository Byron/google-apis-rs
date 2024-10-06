#!/usr/bin/env python

# Generate yaml output suitable for use in shared.yaml
#

import os
import sys

import yaml

isfile = os.path.isfile
isdir = os.path.isdir
join = os.path.join

if __name__ != "__main__":
    raise AssertionError("Not for import")

if len(sys.argv) != 4:
    sys.stderr.write(
        "USAGE: <program> <api_dir> <api-list.yaml> <dest.yaml>, i.e. <program> etc/api etc/api/api-list.yaml out.yaml\n"
    )
    sys.exit(1)

api_base = sys.argv[1]
if not isdir(api_base):
    raise ValueError("Directory '%s' not accessible" % api_base)

yaml_path = sys.argv[2]
if isfile(yaml_path):
    api_data = yaml.load(open(yaml_path, "r"), Loader=yaml.FullLoader)["api"]["list"]
else:
    api_data = dict()


for api_name in sorted(os.listdir(api_base)):
    api_path = join(api_base, api_name)
    if not isdir(api_path):
        continue
    all_versions = sorted(
        (
            v
            for v in os.listdir(api_path)
            if isdir(join(api_path, v))
            and isfile(join(api_path, v, "%s-api.json" % api_name))
        ),
        reverse=True,
    )
    if not all_versions:
        try:
            del api_data[api_name]
        except:
            continue
        continue
    last_version = None
    for v in all_versions:
        if "beta" not in v and "alpha" not in v:
            last_version = v
            break
    # end for each version
    if last_version is None:
        last_version = all_versions[0]

    versions = api_data.get(api_name, list())
    if last_version not in versions:
        versions.append(last_version)
    version = list(sorted(set(versions) & set(all_versions)))
    if versions:
        api_data[api_name] = versions
    else:
        del api_data[api_name]
# end for each item in api-base

fp = open(sys.argv[3], "wt")
fp.write("# DO NOT EDIT !!!\n")
fp.write("# Created by '%s'\n" % " ".join(sys.argv))
fp.write("# DO NOT EDIT !!!\n")
yaml.dump(dict(api=dict(list=api_data)), fp, default_flow_style=False)
fp.close()
