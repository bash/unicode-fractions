#!/usr/bin/env bash
set -e

rm -rf _site/
mkdir _site/
cp -r *.css *.js *.html fonts/ _site/
