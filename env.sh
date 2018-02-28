#!/bin/sh

# Absolute path to this script, e.g. /home/user/bin/foo.sh
SCRIPT=$(readlink -f "$0")
# Absolute path this script is in, thus /home/user/bin
SCRIPTPATH=$(dirname "$SCRIPT")

export PATH=/opt/rust/bin:/opt/riscv/bin:~/.cargo/bin:$PATH
export XARGO_RUST_SRC=~/repos/rust/src
export RUST_TARGET_PATH=$SCRIPTPATH
export TOOLCHAIN=/opt/rust
export LD_LIBRARY_PATH=/opt/riscv/lib
