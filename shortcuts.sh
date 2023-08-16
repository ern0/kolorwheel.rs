#!/bin/bash

alias s="clear ; target/debug/examples/main"
alias p="clear ; cargo build --example main"

#alias t="clear ; cargo test --lib -- --nocapture tst"
#alias t="clear ; cargo test --lib -- --nocapture"
alias t="clear ; cargo test --example main -- --nocapture"
