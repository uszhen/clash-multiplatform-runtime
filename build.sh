#!/bin/bash

cd "$(dirname "$0")" || exit 1

pushd starter || exit 1

cargo build --release || exit 1

popd || exit 1

jlink --add-modules java.base,java.desktop \
  --output jre \
  --compress=2 --ignore-signing-information \
  --no-header-files --no-man-pages --strip-debug \
  --verbose || exit 1

