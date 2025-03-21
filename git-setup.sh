#!/usr/bin/bash

pushd
cd .git/hooks
ln -sf ../../.githooks/* .
popd

