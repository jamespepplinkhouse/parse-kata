#!/bin/bash
set -e

echo 
echo -e "\033[0;33mbash-jq\033[0m"
echo
diff samples/10mb-output.txt data/10mb-bash-jq.txt | diffstat

echo 
echo -e "\033[0;33mbash-sed\033[0m"
echo
diff samples/10mb-output.txt data/10mb-bash-sed.txt | diffstat

echo 
echo -e "\033[0;33mdotnet\033[0m"
echo
diff samples/10mb-output.txt data/10mb-dotnet.txt | diffstat

echo 
echo -e "\033[0;33mgo\033[0m"
echo
diff samples/10mb-output.txt data/10mb-go.txt | diffstat