#!/bin/bash
echo "NOTE: I assume you have called the respective make cargo-cli ARGS=build beforehand"
version=${1:?First argument must be the version CLI we deploy, like v0.1.0}
rtype=${2:?Second argument is either 'release' or 'debug'}

tar_basename=google-apis-rs_cli-${version}_linux-x86-64_${rtype}
tar_file=${tar_basename}.tar.gz
tar -czf $tar_file --transform "s%^.*/%%" -T <(find target -executable -type f -path "*/${rtype}/*" -not \( -name "*.*" -or -name "*script*" -or -regex ".*-[a-f0-9]{16}" \)) || exit $?
echo Wrote $tar_file
