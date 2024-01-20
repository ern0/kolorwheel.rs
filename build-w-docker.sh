#!/bin/bash

TMP=/tmp/build-tmp
mkdir -p $TMP

docker run --init -it \
	-v `pwd`:/prj \
	-v $TMP:/cargo-home \
	--rm rustdev \
	bash -c "cd /prj ; CARGO_HOME=/cargo-home cargo build --release --example main"
