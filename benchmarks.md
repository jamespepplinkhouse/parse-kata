# Benchmarks

## Overview

These are not proper benchmarks!

## Results

| Solution                    | Time (s) | Peak Memory (KB) | Notes                   |
| --------------------------- | -------: | ---------------: | ----------------------- |
| dotnet-multi-chunks-custom  |      5.4 |          1163488 | <ul><li>Buggy</li></ul> |
| rust-single-lines-json      |      8.8 |             2256 |                         |
| dotnet-single-chunks-custom |     10.7 |           192832 | <ul><li>Buggy</li></ul> |
| dotnet-multi-lines-json     |       17 |            63184 |                         |
| go-single-lines-json        |       20 |            15108 |                         |
| nodejs-single-lines-json    |       71 |            93968 |                         |
| python-single-lines-json    |      110 |            18272 |                         |
| python-single-chunks-json   |      196 |           240608 |                         |
| bash-single-lines-json      |      263 |             4176 |                         |
| zig-single-lines-json       |        0 |                  |                         |
