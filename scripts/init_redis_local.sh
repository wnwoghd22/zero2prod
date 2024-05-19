#!/usr/bin/env bash
set -x
set -eo pipefail

docker run \
    -p "6379:6379" \
    -d \
    --name "redis_$(date '+%s')" \
    redis:6

>&2 echo "Redis is ready to go!"