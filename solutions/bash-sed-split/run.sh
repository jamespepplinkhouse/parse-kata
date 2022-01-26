#!/bin/bash

input=../../data/input.txt
num_files=$(sysctl -n hw.ncpu)
total_lines=$(wc -l <$input)
((lines_per_file = (total_lines + num_files - 1) / num_files))

echo "num_files: $num_files"
echo "total_lines: $total_lines"
echo "lines_per_file: $lines_per_file"

# split
split -l ${lines_per_file} $input ../../data/bash-sed-split.

# extract titles concurrently
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.aa | sort > ../../data/bash-sed-split.parsed.aa &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.ab | sort > ../../data/bash-sed-split.parsed.ab &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.ac | sort > ../../data/bash-sed-split.parsed.ac &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.ad | sort > ../../data/bash-sed-split.parsed.ad &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.ae | sort > ../../data/bash-sed-split.parsed.ae &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.af | sort > ../../data/bash-sed-split.parsed.af &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.ag | sort > ../../data/bash-sed-split.parsed.ag &
sed -n "s/^.*\"title\": \"\([^\"]*\)\".*$/\1/p" ../../data/bash-sed-split.ah | sort > ../../data/bash-sed-split.parsed.ah &

# wait for above commands to finish
wait

# join & sort
cat ../../data/bash-sed-split.parsed.aa ../../data/bash-sed-split.parsed.ab ../../data/bash-sed-split.parsed.ac ../../data/bash-sed-split.parsed.ad ../../data/bash-sed-split.parsed.ae ../../data/bash-sed-split.parsed.af ../../data/bash-sed-split.parsed.ag ../../data/bash-sed-split.parsed.ah | sort > ../../data/bash-sed-split-output.txt

# clean up
rm ../../data/bash-sed-split.a*
rm ../../data/bash-sed-split.parsed*