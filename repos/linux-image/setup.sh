#!/bin/bash

LATEST_GIT=false
BRANCH=""

git clone https://github.com/torvalds/linux
if !LATEST_GIT
then
    git checkout $BRANCH
fi

if LATEST_GIT
then
    bpkg metadata set-version ROLLING
else
    bpkg metadata set-version $BRANCH
fi

