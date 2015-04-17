#!/bin/sh
# For execution on docker build image only !
# make cargo-cli ARGS="build --release"
make discovery1-cli-cargo ARGS="build --release"
find gen -executable -type f -path "*/release/*" -not \( -name "*.*" -or -name "*script*" \) | xargs cp -v /build-result