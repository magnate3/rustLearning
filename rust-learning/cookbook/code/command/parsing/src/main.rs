//This application describes the structure of its command-line interface using clap's builder style. The documentation gives two other possible ways to instantiate an application.
//In the builder style, with_name is the unique identifier that value_of will use to retrieve the value passed. The short and long options control the flag the user will be expected to type; short flags look like -f and long flags look like --file.

extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("A cool file"))
        .arg(Arg::with_name("num")
                 .short("n")
                 .long("number")
                 .takes_value(true)
                 .help("Five less than your favorite number"))
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}

/*
cargo run -p parsing -h
cargo-run 
Run a binary or example of the local package

USAGE:
    cargo run [OPTIONS] [--] [args]...

OPTIONS:
    -q, --quiet                      No output printed to stdout
        --bin <NAME>...              Name of the bin target to run
        --example <NAME>...          Name of the example target to run
    -p, --package <SPEC>             Package with the target to run
    -j, --jobs <N>                   Number of parallel jobs, defaults to # of CPUs
        --release                    Build artifacts in release mode, with optimizations
        --profile <PROFILE-NAME>     Build artifacts with the specified profile
        --features <FEATURES>...     Space-separated list of features to activate
        --all-features               Activate all available features
        --no-default-features        Do not activate the `default` feature
        --target <TRIPLE>            Build for the target triple
        --target-dir <DIRECTORY>     Directory for all generated artifacts
        --manifest-path <PATH>       Path to Cargo.toml
        --message-format <FMT>...    Error format
    -v, --verbose                    Use verbose output (-vv very verbose/build.rs output)
        --color <WHEN>               Coloring: auto, always, never
        --frozen                     Require Cargo.lock and cache are up to date
        --locked                     Require Cargo.lock is up to date
        --offline                    Run without accessing the network
    -Z <FLAG>...                     Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    -h, --help                       Prints help information

ARGS:
    <args>...    

If neither `--bin` nor `--example` are given, then if the package only has one
bin target it will be run. Otherwise `--bin` specifies the bin target to run,
and `--example` specifies the example target to run. At most one of `--bin` or
`--example` can be provided.

All the arguments following the two dashes (`--`) are passed to the binary to
run. If you're passing arguments to both Cargo and the binary, the ones after
`--` go to the binary, the ones before go to Cargo.
*/
