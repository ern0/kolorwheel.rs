@echo off
cls

rem cargo test --example main
cargo test --lib -- --nocapture tst
