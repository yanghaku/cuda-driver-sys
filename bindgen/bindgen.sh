#!/bin/bash

set -ex

if [ "${CUDA_HOME}" == "" ]; then
	# default cuda home
	CUDA_HOME="/usr/local/cuda"
fi

DIRNAME=$(dirname "$0")

bindgen \
	--whitelist-type="^CU.*" \
	--whitelist-type="^cuuint(32|64)_t" \
	--whitelist-type="^cudaError_enum" \
	--whitelist-type="^cu.*Complex$" \
	--whitelist-type="^cuda.*" \
	--whitelist-type="^libraryPropertyType.*" \
	--whitelist-var="^CU.*" \
	--whitelist-function="^cu.*" \
	--default-enum-style=rust \
	--no-doc-comments \
	--with-derive-default \
	--with-derive-eq \
	--with-derive-hash \
	--with-derive-ord \
	${DIRNAME}/wrapper.h -- -I${CUDA_HOME}/include \
	>${DIRNAME}/../src/cuda.rs
