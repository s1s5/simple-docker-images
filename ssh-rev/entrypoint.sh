#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め

if getent group $GROUPNAME >/dev/null; then
    :
else
    groupadd -g $GID $GROUPNAME && \
        useradd -s /bin/bash -u $UID -g $GID $USERNAME -d /home/$USERNAME
    cd /home/$USERNAME
fi

exec gosu $USERNAME ssh -N -R $SSH_PORT:localhost:22 $SSH_HOST
