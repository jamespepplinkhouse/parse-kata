#!/bin/bash
set -e

if [ -z ${1+x} ]; then echo "First parameter (input file path) not supplied"; exit 1; fi
if [ -z ${2+x} ]; then echo "Second parameter (ouput file path) not supplied"; exit 1; fi

# Note: for jq, --raw-output doesn't work with --ascii-output, hence I used awk to strip out the enclosing quotes
awk -F'\t' '{print $5}' $1 | jq --ascii-output .title | awk '{print substr($0, 2, length($0) - 2)}' > $2