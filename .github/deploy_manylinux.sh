#!/bin/bash

# easier debugging
pwd
ls -la

rm py-fixed2float/README.md
cp README.md py-fixed2float/README.md
cd py-fixed2float
rustup override set nightly-2022-01-01
maturin publish \
  --skip-existing \
  --username fran
