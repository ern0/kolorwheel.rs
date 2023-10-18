@echo off
cls

cargo rustdoc -- --html-in-header style.css
rem cargo build --example main
rem cargo build
