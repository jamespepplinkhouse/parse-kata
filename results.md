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
- The memory usage of `sed` is amazingly small, considering it's collecting 24M titles; I checked and it's not writing to disk
- `sort` takes 4.66GB of RAM
- The output file has 24,010,895 lines, which is off by one...
