#!/usr/bin/env bash

set -euxo pipefail


# Use local chiptool if available, otherwise use system one
if [ -x ../chiptool/target/release/chiptool ]; then
    CHIPTOOL=../chiptool/target/release/chiptool
elif command -v chiptool &> /dev/null; then
    CHIPTOOL=chiptool
else
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

rm -rf src/chips

export RUST_BACKTRACE=1
export RUST_LOG=info

cat transform.yaml > transform-compat.yaml
cat transform-extra.yaml >> transform-compat.yaml

#for patch in $(ls svd-patches); do
#	patch -R -p1 < svd-patches/$patch
#done

#for chip in nrf52840.svd; do
#for chip in nrf52840.svd nrf54l15-app.svd; do
for chip in $(ls svd); do
    chip=${chip%.*}

    if [[ "$chip" == *nrf54* ]]; then
        $CHIPTOOL generate --svd svd/$chip.svd --transform transform.yaml
    else
        $CHIPTOOL generate --svd svd/$chip.svd --transform transform-compat.yaml
    fi

    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs

    mkdir -p src/chips/$chip
    mv lib.rs src/chips/$chip/pac.rs
    mv device.x src/chips/$chip/device.x
done

cargo fmt
for chip in $(ls svd); do 
    chip=${chip%.*}
    cargo check --features $chip
    cargo doc --features $chip
done
