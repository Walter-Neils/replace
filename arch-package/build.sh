#!/bin/bash
rm -rf ./build
mkdir build
cp -R ../target/release/replace ./build/
cp PKGBUILD ./build/
cd build
makepkg -s
