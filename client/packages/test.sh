#!/usr/bin/env bash -e
cd cli; ts-node index.ts
yarn test

cd executor; yarn test
