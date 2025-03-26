#!/bin/sh

cd .git/hooks
ln -sf ../../.githooks/prepare-commit-msg .
rm prepare-commit-msg.sample
cd ../../

