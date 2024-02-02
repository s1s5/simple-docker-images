#!/bin/bash
# -*- mode: shell-script -*-

ss -tuln | grep LISTEN | grep ":$LOCAL_PORT"
exit $?
