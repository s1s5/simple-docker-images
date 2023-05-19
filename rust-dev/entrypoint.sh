#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


groupadd -g $GROUP_ID $GROUP_NAME
useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID $USER_NAME

mkdir -p /home/${USER_NAME}
export HOME=/home/${USER_NAME}

exec gosu $USER_NAME "$@"
