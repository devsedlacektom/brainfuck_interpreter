use clap::{App, ArgMatches};
use std::fs::File;

/// Initialize commandline arguments parser and parse arguments
pub fn cli_init() -> ArgMatches {
    App::new("brainfuck-interpreter")
        .author("Tomáš Sedláček - xsedlac8")
        .about("Allows to interpret a brainfuck program.")
        .arg("-f, --file=[FILE] 'specifies which file to load'")
        .get_matches()
}

/// Get file path from parsed arguments
pub fn get_file_path(arg_matches: ArgMatches) -> String {
    let result = match arg_matches.value_of("file") {
        Some(file_path) => file_path,
        _ => panic!("No file path was given, terminating the program."),
    };
    String::from(result)
}

/// Open desired file and pass the handle to the caller
pub fn open_desired_brainfuck_file(file_path: String) -> File {
    let desired_file = File::open(file_path);
    match desired_file {
        Ok(file) => file,
        _ => panic!("Cannot open desired file. terminating program"),
    }
}
