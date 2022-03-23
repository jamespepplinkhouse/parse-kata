# Go Notes

## Profiling

### Install dependencies

```sh
brew install graphviz # needed for interactive web
go get -u github.com/google/pprof
```

### Instrument code

- Import "github.com/pkg/profile" to main
- Add `defer profile.Start(profile.ProfilePath(".")).Stop()` to top of main fn

### Analyse profile

```sh
go tool pprof -http :8080 ./cpu.pprof # web
go tool pprof -png ./cpu.pprof # generate a png
```
