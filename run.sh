#!/bin/bash

set -x

#while true; do
  #date
  #inotifywait -e modify ${1}.so
  #date
  #echo "Reloading ${1}.so"
  #redis-cli config set module-add `pwd`/${1}.so
#done

cargo build && \
cp target/debug/libpong.so . && \
redis-cli config set module-add $(pwd)/libpong.so && \
redis-cli rust
