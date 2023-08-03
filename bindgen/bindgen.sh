#!/bin/bash

set -ex

if [ "${CUDA_HOME}" == "" ]; then
	# default cuda home
	CUDA_HOME="/usr/local/cuda-11.7"
fi

DIRNAME=$(dirname "$0")

bindgen \
	--allowlist-type="^CU.*" \
	--allowlist-type="^cuuint(32|64)_t" \
	--allowlist-type="^cudaError_enum" \
	--allowlist-type="^cu.*Complex$" \
	--allowlist-type="^cuda.*" \
	--allowlist-type="^libraryPropertyType.*" \
	--allowlist-var="^CU.*" \
	--allowlist-function="^cu.*" \
	--default-enum-style=rust \
	--no-doc-comments \
	--with-derive-default \
	--with-derive-eq \
	--with-derive-hash \
	--with-derive-ord \
	${DIRNAME}/wrapper.h -- -I${CUDA_HOME}/include \
	>${DIRNAME}/../src/cuda.rs
