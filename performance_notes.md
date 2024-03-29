# Rust - Byte chunks approach - Input file read only; no parsing, or output file

```
./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o -f 0.97s user 1.95s system 60% cpu 4.810 total
```

# Rust - Lines of String approach - Input file read only; no parsing, or output file

```
./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o 3.88s user 1.68s system 93% cpu 5.946 total
```

# Rust - Lines of String by read_until - Input file read only; no parsing, or output file

```
./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o 1.07s user 2.22s system 67% cpu 4.906 total
```

# Rust - Lines of String by read_until - Input file read; find JSON start index, no output file

```
./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o    1.59s  user 2.21s system 71% cpu 5.311 total
```

# Rust - Lines of bytes by read_until - Input file read; producer-consumer model

1.48x slower than single threaded read_until

```
./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o    54.41s  user 10.13s system 83% cpu 1:17.24 total
```

# Rust - Lines of bytes, use simd_json only where required

```
./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o    8.01s  user 2.55s system 95% cpu 11.049 total
```

# C# - Byte chunks approach - Input file read only; no parsing, or output file

```
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 0.07s user 1.96s system 44% cpu 4.509 total
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 0.07s user 1.87s system 43% cpu 4.420 total
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 0.07s user 1.89s system 44% cpu 4.424 total
```

# C# - Lines of String approach - Input file read only; no parsing, or output file

```
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 56.90s user 4.43s system 278% cpu 22.019 total
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 21.86s user 4.50s system 120% cpu 21.949 total
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 21.81s user 4.47s system 120% cpu 21.787 total
```

# C# - Byte chunks approach - Full

```
./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 6.51s user 2.54s system 187% cpu 4.818 total
```

# C# - Notes

- StreamBytes IEnumerable is very fast, it pulls new file chunks quickly (before other pieces are processed), currently does not correct the chunk tails
- Suggest combining the faster byte chunk approach with the JSON parser, to see the overall impact; i.e. the custom algorithm is hard to maintain

# Rust - Notes

- Try a single threaded approach with SIMD for searching and parsing

- Set up consumers (based on machine CPU count) with a callback to get next job
- Each job has an integer ID (chunk #), and the byte array

# Rust, simd-json

./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o 59.26s user 3.09s system 100% cpu 1:01.81 total

# Rust, PGO simd-json

./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o 55.14s user 3.07s system 101% cpu 57.554 total

# Rust, serde_json

./target/release/parse-kata -i ../../data/ol_dump_works_2022-09-30.txt -o 95.67s user 3.30s system 97% cpu 1:41.13 total

# C# - full, multiple cores

24s

# C# - full, multi-core, .NET 8.0

./bin/Release/net8.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 48.27s user 7.05s system 323% cpu 17.089 total

# C# - full, single core

./bin/Release/net7.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 48.33s user 4.06s system 98% cpu 52.920 total

# Python - full, lines, json

./parse-kata.py ../../data/ol_dump_works_2022-09-30.txt 102.66s user 7.03s system 99% cpu 1:50.10 total

# bash-single-lines-json

./run.sh ~/git/parse-kata/data/ol_dump_works_2022-09-30.txt 438.35s user 13.66s system 171% cpu 4:23.97 total

# C# - dotnet-multi-chunks-custom - single thread

./bin/Release/net8.0/ParseKata -i ../../../data/ol_dump_works_2022-09-30.txt 5.41s user 2.42s system 73% cpu 10.707 total

# python-single-chunks-json

./parse-kata.py ../../data/ol_dump_works_2022-09-30.txt 196.21s user 9.53s system 95% cpu 3:35.22 total
avg shared (code): 0 KB
avg unshared (data/stack): 0 KB
total (sum): 0 KB
max memory: 240608 KB
page faults from disk: 54
other page faults: 992474
