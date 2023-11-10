#!/bin/sh
echo "Benchmarking"


hyperfine --prepare 'node gen-big-folders.js' 'rm -rf testings/*' "./target/release/rust-rm 'testings/*'" --warmup 3 --runs 10