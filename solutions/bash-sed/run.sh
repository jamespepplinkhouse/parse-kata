#!/bin/bash
set -e
cd "$(dirname "$0")"

if [ -z ${1+x} ]; then echo "First parameter (input file path) not supplied"; exit 1; fi
if [ -z ${2+x} ]; then echo "Second parameter (ouput file path) not supplied"; exit 1; fi

sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" $1 > $2

# -n               suppress printing
# s                substitute
# ^.*              anything before `"title": "`
# \"title\": \"    initial search match
# \(               start capture group
# [^\"]*           capture everything until the next double quote
# \)               end capture group
# .*$              anything after the capture group, to the end of the line
# \1               substitute everything with the 1st capture group
# p                print it
