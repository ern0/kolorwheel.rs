#!/bin/bash

alias s="clear ; RUST_BACKTRACE=1 target/debug/examples/main"
alias p="clear ; cargo build --examples"
#alias t="clear ; cargo test --examples"
alias t="clear ; cargo test --nocapture" 
