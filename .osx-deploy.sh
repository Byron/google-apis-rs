#!/bin/bash
echo "NOTE: I assume you have called the respective make cargo-cli ARGS=build beforehand"
version=${1:?First argument must be the version CLI we deploy, like v0.1.0}
rtype=${2:?Second argument is either 'release' or 'debug'}

tar_basename=google-apis-rs_cli-${version}_osx-10.10_${rtype}
tar_file=${tar_basename}.tar.gz
dest_dir=build/$tar_basename
mkdir -p $dest_dir || exit $?
find -E gen -perm +0111 -type f -path "*/${rtype}/*" -not \( -name "*.*" -or -name "*script*" -or -regex ".*-[a-f0-9]{16}" \) | xargs -J % cp -v % $dest_dir || exit $?
(cd build && tar -czvf ${tar_file} ${tar_basename}) || exit $?
rm -Rfv $dest_dir || exit $?
echo Wrote build/$tar_file
