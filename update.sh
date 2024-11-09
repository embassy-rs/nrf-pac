#!/usr/bin/env bash

if ! command -v chiptool &> /dev/null; then
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

if ! command -v form &> /dev/null; then
    echo "form could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install form"
    echo ""
    exit 1
fi

set -euxo pipefail

rm -f src/chips/*/*.rs

(cd ../chiptool/; cargo build)
export RUST_BACKTRACE=1
export RUST_LOG=info
chiptool=../chiptool/target/debug/chiptool

for chip in $(ls svd); do 
    chip=${chip%.*}
    $chiptool generate --svd svd/$chip.svd --transform transform.yaml
    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs
    mv lib.rs src/chips/$chip/pac.rs
done

cargo fmt
for chip in $(ls svd); do 
    chip=${chip%.*}
    cargo check --features $chip
    cargo doc --features $chip
done
