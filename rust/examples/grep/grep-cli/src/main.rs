use clap::{crate_authors, crate_version, App, Arg};
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::path::Path;
use std::thread;

fn main() {
    let matches = App::new("grep")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Search for PATTERNS in each FILES.")
        .arg(
            Arg::with_name("fixed-strings")
                .short("F")
                .long("fixed-strings")
                .help("PATTERNS are strings"),
        )
        .arg(
            Arg::with_name("PATTERNS")
                .help("use PATTERNS for matching")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILES")
                .help("take PATTERNS from FILES")
                .required(true)
                .multiple(true)
                .index(2),
        )
        .get_matches();

    let pattern = matches.value_of("PATTERNS").unwrap();
    let file_pathes = matches
        .values_of("FILES")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let is_fixed_strings_mode = matches.is_present("fixed_strings");
    println!("{:?}", matches);

    for file_path in file_pathes {
        let path = Path::new(&file_path);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
            Ok(file) => file,
        };
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
            Ok(_) => {
                for line in s.lines() {
                    println!("{}", line);
                }
            }
        }
    }
}
