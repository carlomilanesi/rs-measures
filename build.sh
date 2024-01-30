#!/bin/sh
clear &&\
cargo clean &&\
./format_measures.sh &&\
cargo clippy --all-targets -- -D warnings &&\
cargo test &&\
echo OK
