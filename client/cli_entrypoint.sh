#!/bin/bash

# This script is the entrypoint for the CLI container.
pnpm run cli init -c
pnpm run cli init -t
pnpm run cli "$@"
