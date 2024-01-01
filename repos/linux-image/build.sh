#!/bin/bash

cd linux
make -j $(nproc)

# (TODO) Create new directory and move all files that needs to be installed there
