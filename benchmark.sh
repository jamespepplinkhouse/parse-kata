#!/bin/bash
set -e

hyperfine --warmup 3 \
    -n bash-jq 'solutions/bash-jq/run.sh samples/10mb-input.txt data/10mb-bash-jq.txt' \
    -n dotnet-json 'solutions/dotnet/ParseKata/bin/Release/net7.0/ParseKata -i samples/10mb-input.txt -o data/10mb-dotnet.txt' \
    -n dotnet-custom 'solutions/dotnet/ParseKata/bin/Release/net7.0/ParseKata -i samples/10mb-input.txt -o data/10mb-dotnet.txt -f' \
    -n golang 'solutions/go/parse-kata samples/10mb-input.txt data/10mb-go.txt' \
    -n nodejs 'solutions/nodejs/build/index.js -i samples/10mb-input.txt -o data/10mb-nodejs.txt'

