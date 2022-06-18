# dotnet

## Installation

```sh
brew install --cask dotnet-sdk
dotnet --list-runtimes
dotnet --list-sdks

dotnet tool install --global dotnet-trace -a arm64
```

## Project Setup

```sh
dotnet new sln -o dotnet
cd dotnet
dotnet new classlib -o ParseKata
mv ./ParseKata/Class1.cs ./ParseKata/ParseKata.cs
dotnet sln add ./ParseKata/ParseKata.csproj
dotnet new xunit -o ParseKata.Tests
dotnet add ./ParseKata.Tests/ParseKata.Tests.csproj reference ./ParseKata/ParseKata.csproj
dotnet sln add ./ParseKata.Tests/ParseKata.Tests.csproj
```

## Building / Running

```sh
dotnet run ../../data/input.txt ../../data/dotnet.txt

dotnet build --configuration Release
```

## Tracing

```sh
dotnet-trace collect --output trace --format Chromium -- ./bin/Release/net6.0/dotnet ../../data/input.txt ../../data/dotnet.txt
dotnet-trace collect --output trace --format Speedscope -- ./bin/Release/net6.0/dotnet ../../data/input.txt ../../data/dotnet.txt
```

For `Chromium` trace format, in Chrome navigate to: [chrome://tracing/](chrome://tracing/)

For `Speedscope` trace format, navigate to: https://www.speedscope.app/

I recommend Speedscope!

![Speedscope Sandwich](./docs/speedscope-sandwich.png)
![Speedscope Left Heavy](./docs/speedscope-left-heavy.png)
