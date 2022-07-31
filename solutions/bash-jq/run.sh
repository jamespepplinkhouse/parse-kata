#!/bin/bash
set -e
cd "$(dirname "$0")"

if [ -z ${1+x} ]; then echo "First parameter (input file path) not supplied"; exit 1; fi
if [ -z ${2+x} ]; then echo "Second parameter (ouput file path) not supplied"; exit 1; fi

awk -F'\t' '{print $5}' $1 | jq -r .title > $2