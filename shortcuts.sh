#!/bin/bash

alias s="clear ; target/debug/examples/main"
alias p="clear ; cargo build --example main"

#alias t="clear ; cargo test --example main ; cargo test --lib"
#alias t="clear ; cargo test --lib -- --nocapture tst"
alias t="clear ; cargo test --lib -- --nocapture"
