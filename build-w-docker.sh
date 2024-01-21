#!/bin/bash

# create a Docker volume (faster than bind volume):
#   $ docker volume create cargo-home
# create image:
#   $ docker build -f Dockerfile -t rustdev .

docker run --init -it \
	-v `pwd`:/prj \
	--mount source=cargo-home,target=/cargo-home \
	--rm rustdev \
	bash -c "cd /prj ; CARGO_HOME=/cargo-home cargo build --release --example main"
