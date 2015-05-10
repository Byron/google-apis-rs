#!/bin/sh
# For execution on docker build image only !
# make cargo-cli ARGS="build --release"
# DEBUG: only try to build a small CLI for now
make discovery1-cli-cargo ARGS="build --release"
find gen -executable -type f -path "*/release/*" -not \( -name "*.*" -or -name "*script*" \) | xargs -J % cp -v % /build-result