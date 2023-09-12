#!/bin/bash

# TODO: this probably can be removed now and defined as ENTRYPOINT
# This script is the entrypoint for the CLI container.
pnpm run cli "$@"
