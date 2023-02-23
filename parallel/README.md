# Parallel Rust with Rayon

A program to walk the directory tree and return duplicate files. Walk can be executed in serial or in parallel (with user defined thread option).
## Useage

```
$ cd parallel
```

**Serial execution**
```
$ cargo run serial --path <data_dir>
```

**Parallel execution**
```
# Default number of threads
$ cargo run parallel --path <data_dir>

# For user defined number of threads
$ cargo run parallel --path <data_dir> --threads <num_threads>
```

**Run Tests**
```
$ make test
```