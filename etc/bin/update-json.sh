#!/bin/bash

repo_path=${1:?First argument must be the part to the google go API clients repository}
api_base=${2:?Second argument must be the destination path to which to copy the APIs}

(cd ${repo_path} && git pull --ff-only) || exit $?

for json_path in `cd ${repo_path} && find . -type f  -name "*-api.json"`; do
	dest=${api_base}/`dirname ${json_path}`
	mkdir -p ${dest} || exit $?
	cp ${repo_path}/${json_path} ${dest} || exit $?
done