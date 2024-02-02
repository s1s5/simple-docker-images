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

echo gosu $USERNAME ssh -N -L $LOCAL_PORT:localhost:$REMOTE_PORT $REMOTE_HOST
exec gosu $USERNAME ssh -N -L $LOCAL_PORT:localhost:$REMOTE_PORT $REMOTE_HOST
