#!/usr/bin/env bash -e
find . -type d -name node_modules | xargs rm -rf {}\;

cd types; yarn; yarn build;
cd ../sdk; yarn; yarn build;
cd ../cli; yarn add ../sdk; yarn;
cd ../executor; yarn add ../sdk; yarn add ../types; yarn upgrade ../sdk; yarn upgrade ../types; yarn;

cd ..

