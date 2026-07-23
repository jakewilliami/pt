<h1 align="center">pt</h1>
<h3 align="center"><i>When is the penultimate Tuesday of the month?</i></h3>

## Description

A simple command line tool to display the date of the next penultimate Tuesday of the month (for [no particular reason](https://mathsjam.com/)).

The calculation of the dates is ported from [PenultimateDays.jl](https://github.com/jakewilliami/PenultimateDays.jl).

## Quick Start

### CLI

```commandline
$ just
$ ./pt -h
```

### Library

```rust
use chrono::{Weekday, Local};
use pt::penultimate_day_of_month;

let today = Local::now().date_naive();
let penultimate_tuesday = penultimate_day_of_month(today, Weekday::Tuesday);
```

## Usage

```commandline
$ pt -h
When is the next penultimate Tuesday of the month?

Usage: pt

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```
