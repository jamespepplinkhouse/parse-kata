#!/bin/bash
set -e
ROOT=$(pwd)

echo 
echo -e "\033[0;33mBuilding dotnet solution\033[0m"
echo
cd $ROOT/solutions/dotnet/ParseKata
time dotnet build --configuration Release

echo 
echo -e "\033[0;33mBuilding go solution\033[0m"
echo
cd $ROOT/solutions/go
time go build -v

echo 
echo -e "\033[0;33mBuilding Node.js solution\033[0m"
echo
cd $ROOT/solutions/nodejs
time yarn build