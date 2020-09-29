#!/bin/bash

echo "This script updates the dependent repo github.com/t3rn/gateway-pallet that builds dependent on."
echo "Type path root path to main pallet-escrow-gateway sources (relative or absolute)"
read -r SRC_PATH
echo "Type your commit message, followed by [ENTER]:"
read -r COMMIT_MSG

cd $SRC_PATH || exit
ABS_SRC_PATH=`pwd`

DEP_REPO_PATH="/tmp/gateway-pallet"
rm -rf $DEP_REPO_PATH
git clone https://github.com/t3rn/gateway-pallet $DEP_REPO_PATH

cd $DEP_REPO_PATH || exit

git rm -rf .
git clean -fxd

rsync -av $ABS_SRC_PATH/ $DEP_REPO_PATH/ --exclude=target

git add .
git status
git commit -S -m "$COMMIT_MSG"

rm -rf $DEP_REPO_PATH
