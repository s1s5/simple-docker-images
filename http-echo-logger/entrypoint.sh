#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め

#!/bin/bash

# Start the first process
/app --bind 0.0.0.0:80 --server 127.0.0.1:8888 --without-outbound &

# Start the second process
gunicorn -b 127.0.0.1:8888 httpbin:app -k gevent &

# Wait for any process to exit
wait -n

# Exit with status of process that exited first
exit $?
