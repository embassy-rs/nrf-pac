#!/usr/bin/env bash

set -euxo pipefail


if ! command -v chiptool &> /dev/null; then
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

rm -rf src/chips

export RUST_BACKTRACE=1
#export RUST_LOG=info

for chip in $(ls svd); do
    chip=${chip%.*}

    if [ "$chip" = "nrf54l15-flpr" ]; then
        target="riscv"
    else
        target="cortex-m"
    fi
    chiptool generate --svd svd/$chip.svd --transform transform.yaml --target $target
    rustfmt lib.rs

    sed -i '' '/#!\[no_std]/d' lib.rs

    mkdir -p src/chips/$chip
    mv lib.rs src/chips/$chip/pac.rs
    mv device.x src/chips/$chip/device.x
done

cargo fmt

# Actual target doesn't matter for the check, but it must be a valid Cortex-M target
cortex_m_target="thumbv7em-none-eabihf"
riscv_target="riscv32emc-unknown-none-elf"


for chip in $(ls svd); do
    chip=${chip%.*}

    if [ "$chip" = "nrf54l15-flpr" ]; then
        target=$riscv_target
        additional_args="-Zbuild-std=core,compiler_builtins"
    else
        target=$cortex_m_target
        additional_args=""
    fi


    cargo check --features $chip --target $target $additional_args
    cargo doc --features $chip --target $target $additional_args
done
