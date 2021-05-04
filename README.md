# `cargo advent`

## A cargo plugin for fetching advent of code inputs

Open each "door" in style this advent

```sh
$ cargo advent
fetching https://adventofcode.com/2021/day/1
input saved to ./input/2021/day1.txt
```

This is a _very_ minimalist alternative to cargo-aoc.

This does not provide any macros or code running facilities, and no benchmarks.

This has one job: download the input for a given day of advent of code and store it in a file.

To do this it stores your session token, just grab it from the browser and set it like so:

```sh
$ cargo advent --set-credentials TOKEN
credentials saved to /home/drmason13/.config/cargo-advent/session_token
```

You might also be interested in a companion [`Makefile.toml`](./Makefile.toml) ***Work in Progress*** that can be run by the excellent `cargo make` to streamline use of this plugin, as well as building, testing and running your solutions.

You will be able to use it in your own projects by copying it verbatim to a `Makefile.toml` located in the root of your crate; or you can [expand]() your own `Makefile.toml` to use it in amongst your own workflow.

You will of course need to install `cargo-make` first, but once you do, installation of `cargo-advent` is automatically handled for you by running cargo make.

## FAQ

* Why isn't this running my code for me?

It isn't supposed to. You will probably enjoy using gobanos' [cargo-aoc](https://github.com/gobanos/cargo-aoc), which is a direct inspiration for this project, thank you gobanos.

* I've downloaded my advent of code input... now what?

Now you can start coding your solution!

If you're stuck on reading the downloaded input files, I recommend consulting [rust_by_example](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html).

If that seems like too much hassle, I'd definitely recommend [cargo-aoc](https://github.com/gobanos/cargo-aoc), it leaves you free to focus on the logic of the puzzle instead of file IO.

* This isn't working for me it gives me a weird error about paths?

This tool doesn't support non-UTF8 paths.

It is possible using OSStrings but a conscious decision was made not to do this... Let me know if it is a problem for you and let's fix it.

## Examples

### Store your session token at the default location
```sh
$ cargo advent --set-credentials <your token>
credentials saved to /home/drmason13/.config/cargo-advent/session_token
```

### Get today's advent of code input (only works during advent of code)
```sh
$ cargo advent
```

### Get the input for day 4 of the most recent advent of code
```sh
$ cargo advent --day 4
```

### Get a previous advent of code input
```sh
$ cargo advent -d 4 --year 2015
```

### Get a previous advent of code input and output to an alternative location (./input/2020-day-04), with verbose output enabled
```sh
$ cargo advent -d 4 -y 2015 --output ./input/2015-day-04 -v
Fetching https://adventofcode.com/2015/day/4/input
Input saved to ./input/2015-day-04
```

## Roadmap

* [ ] warn if not in a cargo project when running (likely to prevent mistaken extra downloads)
* [ ] support using ENV VARIABLES in place of most if not all command line options - this will ease interoperability with cargo make and custom scripts
* [ ] support interpolating {DAY} and {YEAR} in the --output option
* [ ] support downloading all available inputs (not already downloaded of course)
* [ ] system-wide caching of downloaded inputs, to avoid downloading the same input multiple times just to move where you store it in a particular project.
      I'm not sure how desirable this is, but it will be interesting to implement when you take into consideration the possibility of different inputs per session token.

## TODO
* [ ] test suite and continuous integration workflow
* [ ] publish to crates.io