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
dotnet new console -o ParseKata -f net7.0
mv ./ParseKata/Class1.cs ./ParseKata/ParseKata.cs
dotnet sln add ./ParseKata/ParseKata.csproj

dotnet new xunit -o ParseKata.Tests -f net7.0
dotnet add ./ParseKata.Tests/ParseKata.Tests.csproj reference ./ParseKata/ParseKata.csproj
dotnet sln add ./ParseKata.Tests/ParseKata.Tests.csproj

dotnet new console -o ParseKata.Benchmark -f net7.0
dotnet add ./ParseKata.Benchmark/ParseKata.Benchmark.csproj reference ./ParseKata/ParseKata.csproj
dotnet sln add ./ParseKata.Benchmark/ParseKata.Benchmark.csproj

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

## Benchmarking

```sh
cd ParseKata.Benchmark
dotnet build --configuration Release
./bin/Release/net7.0/ParseKata.Benchmark
```

Example output:

```
// * Detailed results *
LogicBenchmark.ExtractTitles: DefaultJob
Runtime = .NET 7.0.0 (7.0.22.30112), Arm64 RyuJIT; GC = Concurrent Workstation
Mean = 1.450 ms, StdErr = 0.006 ms (0.44%), N = 18, StdDev = 0.027 ms
Min = 1.428 ms, Q1 = 1.431 ms, Median = 1.440 ms, Q3 = 1.454 ms, Max = 1.509 ms
IQR = 0.023 ms, LowerFence = 1.396 ms, UpperFence = 1.490 ms
ConfidenceInterval = [1.425 ms; 1.475 ms] (CI 99.9%), Margin = 0.025 ms (1.73% of Mean)
Skewness = 1.11, Kurtosis = 2.63, MValue = 2
-------------------- Histogram --------------------
[1.421 ms ; 1.473 ms) | @@@@@@@@@@@@@@
[1.473 ms ; 1.512 ms) | @@@@
---------------------------------------------------

// * Summary *

BenchmarkDotNet=v0.13.1, OS=macOS Big Sur 11.4 (20F71) [Darwin 20.5.0]
Apple M1, 1 CPU, 8 logical and 8 physical cores
.NET SDK=7.0.100-preview.5.22307.18
  [Host]     : .NET 7.0.0 (7.0.22.30112), Arm64 RyuJIT
  DefaultJob : .NET 7.0.0 (7.0.22.30112), Arm64 RyuJIT


|        Method |     Mean |     Error |    StdDev |
|-------------- |---------:|----------:|----------:|
| ExtractTitles | 1.450 ms | 0.0251 ms | 0.0268 ms |

// * Hints *
Outliers
  LogicBenchmark.ExtractTitles: Default -> 1 outlier  was  removed (1.54 ms)

// * Legends *
  Mean   : Arithmetic mean of all measurements
  Error  : Half of 99.9% confidence interval
  StdDev : Standard deviation of all measurements
  1 ms   : 1 Millisecond (0.001 sec)
```
