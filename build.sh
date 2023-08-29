#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


export FOLDER=$1
export TAG=`date "+%Y%m%d-%H%M%S"`

docker build ${FOLDER} -t s1s5/${FOLDER}
docker tag s1s5/${FOLDER} s1s5/${FOLDER}:${TAG}
docker push s1s5/${FOLDER}:${TAG}
docker push s1s5/${FOLDER}
