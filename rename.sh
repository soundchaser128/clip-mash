#!/bin/bash
BUILD_TYPE=${1:-release}
if [[ "$BUILD_TYPE" != "release" && "$BUILD_TYPE" != "debug" ]]; then
    echo "Usage: $0 [release|debug]"
    exit 1
fi
TARGET_TRIPLE=$(rustc -Vv | grep host | cut -f2 -d' ')
if [[ "$TARGET_TRIPLE" == *"windows"* ]]; then
    SUFFIX=".exe"
else
    SUFFIX=""
fi
cp "target/$BUILD_TYPE/clip-mash-server" "target/${BUILD_TYPE}/clip-mash-server-${TARGET_TRIPLE}${SUFFIX}"

echo "Renamed binary to target/${BUILD_TYPE}/clip-mash-server-${TARGET_TRIPLE}${SUFFIX}"
