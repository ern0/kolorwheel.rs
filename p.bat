@echo off
cls

rem cargo build
cargo rustdoc -- --html-in-header doc/style.css
rem cargo build --example main
