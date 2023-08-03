#!/bin/bash

set -ex

if [ "${CUDA_HOME}" == "" ]; then
  # default cuda home
  CUDA_HOME="/usr/local/cuda-10.2"
fi

DIRNAME=$(dirname "$0")

bindgen \
  --allowlist-type="^cublas.*" \
  --allowlist-function="^cublas.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  ${DIRNAME}/wrapper_cublas.h \
  -- -I${CUDA_HOME}/include \
  >${DIRNAME}/../src/cublas.rs
