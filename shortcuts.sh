#!/bin/bash

alias s="clear ; target/debug/examples/main"
alias p="clear ; cargo build --example main"
#alias t="clear ; cargo test --examples"
alias t="clear ; cargo test -- --nocapture" 
