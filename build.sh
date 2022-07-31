#!/bin/bash
set -e
ROOT=$(pwd)

echo 
echo "--> Building dotnet solution"
echo
cd $ROOT/solutions/dotnet/ParseKata
time dotnet build --configuration Release

echo 
echo "--> Building go solution"
echo
cd $ROOT/solutions/go
time go build -v