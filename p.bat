@echo off
cls

rem cargo build
rem cargo rustdoc -- --html-in-header doc/style.css
cargo build --example main
