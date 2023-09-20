# Benchmarks

## Overview

These are not proper benchmarks!

## Results

| Solution                   | Time (s) | Peak Memory | Notes                         |
| -------------------------- | -------: | ----------: | ----------------------------- |
| dotnet-multi-chunks-custom |      5.4 |  1163488 KB | <ul><li>Buggy</li></ul>       |
| dotnet-multi-lines-json    |       17 |             |                               |
| go-single-lines-json       |       20 |    15108 KB |                               |
| rust-single-lines-json     |       57 |             | <ul><li>PGO enabled</li></ul> |
| nodejs-single-lines-json   |       71 |    93968 KB |                               |
| python-single-lines-json   |      110 |             |                               |
| bash-single-lines-json     |      263 |             |                               |
| zig-single-lines-json      |        0 |             |                               |
