# Architecture
This document describes the high-level architecture of cargo-advent which does not take long.
If you want to familiarize yourself with the code base, you are in the right place!

## Bird's eye view

```text
binary ---calls--> lib (lib does everything)
```

## Entry Point

`src/main.rs` is a minimum wrapper around the lib, everything is implemented in the library, including argument parsing.

`src/run.rs` is the entrypoint to the lib. It contains `pub fn entrypoint` which is the only function used by main.

## Code Map

### `run.rs`
The entrypoint, this depends on args.rs to parse arguments.

### `args.rs`
Argument parsing happens in `args.rs` in the `Args` struct.

It is a little bit cheeky and short-circuits argument parsing to call the lib early if certain "dead-end" arguments show up.

`Args` depends on `date.rs` when parsing the defaults for --day and --year.

### `date.rs`
Types for validating year and day and whether there is an Advent of Code input available for thea given combination.

--day and --year are deduced and validated based on the current date, all the logic for that is in the `Advent` struct.

Errors in this validation process are described in `error.rs`.

### `error.rs`
All meaningful failures to parse combinations of --day and --year are here in the `AdventError` enum.
