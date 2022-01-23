#!/bin/bash

# easier debugging
pwd
ls -la

rustup override set nightly-2022-01-01
maturin publish \
  --skip-existing \
  --username fran
