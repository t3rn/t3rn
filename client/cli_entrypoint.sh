#!/bin/bash

# This script is the entrypoint for the CLI container.
yarn cli init -c
yarn cli "$@"
