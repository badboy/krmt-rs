#!/bin/bash

set -x

name=libfst_cmd.so

cargo build && \
cp target/debug/$name . && \
redis-cli config set module-add $(pwd)/$name && \
redis-cli rust
