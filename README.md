# SPWM Generator
This is a simple cli app to help you generate `SPWM` lookup table for your projects

## Usage
You can use prebuilt binaries or build the project with rust compiler

## Install with Rust
You can install this tool with `cargo`
```
cargo install spwm_generator
```

## Parameters
```
Usage: spwm_generator [OPTIONS] --sin_freq <SIN_FREQ> --num_of_samples <NUM_OF_SAMPLES> --out <OUTPUT>

Options:
  -f, --sin_freq <SIN_FREQ>
          sin wave frequency
  -n, --num_of_samples <NUM_OF_SAMPLES>
          number of samples
  -t, --pwm_top <PWM_TOP>
          pwm top value [default: 255]
  -p, --padding <PADDING>
          padding for pwm min and max value [default: 0]
  -o, --out <OUTPUT>
          output file path
  -m, --format <FORMAT>
          format [default: raw] [possible values: raw, raw-hex, c, c-hex, rust, rust-hex]
  -w, --row_width <ROW_WIDTH>
          number of samples in row [default: 16]
  -s, --separator <SEPARATOR>
          separator character [default: ", "]
  -a, --name <NAME>
          name of variable [default: WAVE]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

## Example C
Generate `SPWM` table for `C` project
```
spwm_generator -m c -f 50 -c 10000 -o Table_50Hz.c
```
or
```
spwm_generator -m c-hex -f 50 -c 10000 -o Table_50Hz.c
```

## Example Rust
Generate `SPWM` table for `Rust` project
```
spwm_generator -m rust -f 50 -c 10000 -o Table_50Hz.c
```
or
```
spwm_generator -m rust-hex -f 50 -c 10000 -o Table_50Hz.c
```

