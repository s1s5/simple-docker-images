#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


filename=/tmp/loop-test-`date "+%Y%m%d-%H%M%S"`${RANDOM}${RANDOM}
echo $filename
gosu $USERNAME ssh -p $REMOTE_PORT -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o ProxyCommand="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -W %h:%p $REMOTE_HOST" $USERNAME@localhost touch ${filename}

if [ -e ${filename} ]; then
    echo "sucess"
    rm $filename
    exit 0
else
    echo "failed"
    exit 1
fi
