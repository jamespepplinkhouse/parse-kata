# Results

## bash-sed

Run time:

```
./run.sh  567.20s user 8.43s system 101% cpu 9:29.16 total
```

Notes:

- One liner!
- `sed` uses 100% CPU (a single core)
- I could break the file up first and use multiple sed commands (to use more CPU), but the bash implementation will start to get unwieldy IMO; I'll give it a go later
- I considered `jq` but we'd still need to use `sed` to parse out the first part of each line; will try `jq` later
-
