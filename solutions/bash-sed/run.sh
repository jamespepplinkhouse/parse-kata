#!/bin/bash

sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/input.txt | sort > ../../data/bash-output.txt
