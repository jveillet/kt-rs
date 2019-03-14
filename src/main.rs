extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process;

fn main() {
    let matches = App::new("kt")
        .version("0.1.0")
        .author("Jérémie Veillet. @jveillet")
        .about("A drop-in cat replacement written in Rust.")
        .arg(
            Arg::with_name("FILE")
                .help("Concatenate FILE to standard output")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines")
                .takes_value(false)
                .requires("FILE"),
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    let mut line_counter = 0;
                    let stdout = std::io::stdout();
                    let mut handle = std::io::BufWriter::new(stdout);
                    for line in reader.lines() {
                        line_counter += 1;
                        if matches.is_present("number") {
                            writeln!(handle, "{} {}", line_counter, line.unwrap()).unwrap_or_else(
                                |err| {
                                    eprintln!(
                                        "[kt Error] Unable to display the file contents. {:?}",
                                        err
                                    );
                                    process::exit(1);
                                },
                            );
                        } else {
                            writeln!(handle, "{}", line.unwrap()).unwrap_or_else(|err| {
                                eprintln!(
                                    "[kt Error] Unable to display the file contents. {:?}",
                                    err
                                );
                                process::exit(1);
                            });
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[kt Error] Unable to read the file. {:?}", err);
                    process::exit(1);
                }
            }
        } else {
            eprintln!("[kt Error] No such file or directory.");
            process::exit(1);
        }
    }
}
