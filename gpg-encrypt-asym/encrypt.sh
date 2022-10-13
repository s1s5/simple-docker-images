#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め

TEMPORARY_DIR=/tmp/gnupg-`date '+%Y-%m/%Y-%m-%d_%H-%M-%S'`
INPUT=
OUTPUT=
KEY_PATH=

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
        -k|--pubkey)
            KEY_PATH="$2"
            shift # past argument
            shift # past value
            ;;
        -*|--*)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done
if [ ${INPUT} = "" ] ||  [ ${OUTPUT} = "" ] ||  [ ${KEY_PATH} = "" ]; then
   echo "input, output, publick_key must be specified"
   exit 1
fi

mkdir -p ${TEMPORARY_DIR}
chown $(id -u):$(id -g) ${TEMPORARY_DIR}
chmod 700 ${TEMPORARY_DIR}
KEY_ID=`gpg --list-packets < ${KEY_PATH} | awk '$1=="keyid:"{print$2}' | head -n 1`
gpg --batch --no-tty --homedir ${TEMPORARY_DIR} --import ${KEY_PATH}
gpg --batch --no-tty --trust-model always --homedir ${TEMPORARY_DIR} --recipient ${KEY_ID} --encrypt --output - ${INPUT} > ${OUTPUT}
rm -rf ${TEMPORARY_DIR}
