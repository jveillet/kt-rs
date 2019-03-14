extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;

fn main() {
    let matches = App::new("kt")
        .version("0.1.0")
        .author("Jérémie Veillet. @jveillet")
        .about("A drop-in cat replacement written in Rust")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("[kt Error] Unable to read the  file.");
                    let stdout = std::io::stdout(); // get the global stdout entity
                    let mut handle = std::io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {}
                        Err(err) => {
                            eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
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
