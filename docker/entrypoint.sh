#!/bin/sh
set -eu

if [ "$#" -eq 0 ]; then
  set -- serve
fi

exec puffy "$@"

