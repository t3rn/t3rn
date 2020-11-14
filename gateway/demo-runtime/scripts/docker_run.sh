#!/usr/bin/env bash

set -e

echo "*** Start Gateway runtime demo as from a docker image ***"

cd $(dirname ${BASH_SOURCE[0]})/..


# Until the bug mentioned in https://github.com/paritytech/substrate/issues/7466 is resolved,
# build your own image with rust nightly-2020-10-01 version.
docker build . -t t3rn/demo-runtime:nightly-2020-10-01
docker-compose down --remove-orphans
docker-compose run --rm --service-ports dev $@