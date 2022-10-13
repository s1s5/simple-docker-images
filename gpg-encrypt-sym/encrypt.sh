#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め

TEMPORARY_DIR=/tmp/gnupg-`date '+%Y-%m/%Y-%m-%d_%H-%M-%S'`
INPUT=
OUTPUT=
KEY=

while [[ $# -gt 0 ]]; do
    case $1 in
        -i|--input)
            INPUT="$2"
            shift # past argument
            shift # past value
            ;;
        -o|--output)
            OUTPUT="$2"
            shift # past argument
            shift # past value
            ;;
        -k|--key)
            KEY="$2"
            shift # past argument
            shift # past value
            ;;
        -*|--*)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done
if [ ${INPUT} = "" ] ||  [ ${OUTPUT} = "" ] ||  [ ${KEY} = "" ]; then
   echo "input, output, key must be specified"
   exit 1
fi

mkdir -p ${TEMPORARY_DIR}
chown $(id -u):$(id -g) ${TEMPORARY_DIR}
chmod 700 ${TEMPORARY_DIR}
echo -n ${KEY} | gpg --homedir ${TEMPORARY_DIR} --batch --no-tty -o ${OUTPUT} --cipher-algo AES256 --passphrase-fd 0 -c ${INPUT}
rm -rf ${TEMPORARY_DIR}
