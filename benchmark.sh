#!/bin/bash
set -e

hyperfine --warmup 3 \
    'solutions/bash-jq/run.sh samples/10mb-input.txt data/10mb-bash-jq.txt' \
    'solutions/bash-sed/run.sh samples/10mb-input.txt data/10mb-bash-sed.txt' \
    'solutions/dotnet/ParseKata/bin/Release/net7.0/ParseKata samples/10mb-input.txt data/10mb-dotnet.txt' \
    'solutions/go/parse-kata samples/10mb-input.txt data/10mb-go.txt'

