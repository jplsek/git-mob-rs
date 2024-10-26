#!/bin/bash
cargo build --release
set -x
git-solo --version
target/release/git-solo --version
# This assumes a "ts" test user is set
hyperfine --warmup 3 -- 'git-solo' 'target/release/git-solo'
hyperfine --warmup 3 -- 'git-mob ts' 'target/release/git-mob ts'
