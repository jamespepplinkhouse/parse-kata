# Results

## bash-sed

Run time:

```
./run.sh  567.20s user 8.43s system 101% cpu 9:29.16 total
```

Notes:

- One liner!
- `sed` uses 100% CPU (a single core)
- I could break the file up first and use multiple sed commands (to use more CPU); I'll give it a go later
- I considered `jq` but we'd still need to use `sed` to parse out the first part of each line; will try `jq` later
- The memory usage of `sed` is amazingly small, it's only a handful of MB, starting with 1.9MB
- `sort` takes 4.66GB of RAM
- The output file has 24,010,895 lines, which is off by one...

## bash-sed-split

Run time:

```
./run.sh  839.54s user 147.84s system 334% cpu 4:55.22 total
```

Notes:

- After splitting the file into chunks to match the number of CPU cores, uses sed commands concurrently; the sort at the end is a single CPU
- Using 8 CPUs averages out to 334% over the whole program
