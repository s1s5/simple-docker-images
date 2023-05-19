#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


if getent group "$GROUP_NAME" >/dev/null; then
    :
else
    groupadd -g $GROUP_ID $GROUP_NAME
    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID $USER_NAME

    mkdir -p /home/${USER_NAME}
    mkdir -p /home/${USER_NAME}/.cargo
    chown -R ${USER_NAME}:${GROUP_NAME} /home/${USER_NAME}
fi
export HOME=/home/${USER_NAME}
export CARGO_HOME=${HOME}/.cargo

exec gosu $USER_NAME "$@"
