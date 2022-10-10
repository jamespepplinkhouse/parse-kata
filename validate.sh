#!/bin/bash
set -e

echo 
echo -e "\033[0;33mbash-jq\033[0m"
echo
solutions/bash-jq/run.sh samples/10mb-input.txt data/10mb-bash-jq.txt
diff samples/10mb-output.txt data/10mb-bash-jq.txt | diffstat

echo 
echo -e "\033[0;33mbash-sed\033[0m"
echo
diff samples/10mb-output.txt data/10mb-bash-sed.txt | diffstat
solutions/bash-sed/run.sh samples/10mb-input.txt data/10mb-bash-sed.txt

echo 
echo -e "\033[0;33mdotnet json\033[0m"
echo
solutions/dotnet/ParseKata/bin/Release/net7.0/ParseKata -i samples/10mb-input.txt -o data/10mb-dotnet-json.txt
diff samples/10mb-output.txt data/10mb-dotnet-json.txt | diffstat

echo 
echo -e "\033[0;33mdotnet custom\033[0m"
echo
solutions/dotnet/ParseKata/bin/Release/net7.0/ParseKata -i samples/10mb-input.txt -o data/10mb-dotnet-custom.txt -f
diff samples/10mb-output.txt data/10mb-dotnet-custom.txt | diffstat

echo 
echo -e "\033[0;33mgo\033[0m"
echo
solutions/go/parse-kata samples/10mb-input.txt data/10mb-go.txt
diff samples/10mb-output.txt data/10mb-go.txt | diffstat

echo 
echo -e "\033[0;33mNode.js\033[0m"
echo
solutions/nodejs/build/index.js -i samples/10mb-input.txt -o data/10mb-nodejs.txt
diff samples/10mb-output.txt data/10mb-nodejs.txt | diffstat