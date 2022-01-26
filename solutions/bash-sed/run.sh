#!/bin/bash

sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/input.txt | sort > ../../data/bash-sed-output.txt

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
