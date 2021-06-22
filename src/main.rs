use clap::{App, Arg};
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

fn main() {
    let app = App::new("cli").author("jakob").arg(
        Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true),
    );
    let matches = app.get_matches();
    let input: &str = matches
        .value_of("input")
        .expect("input value was not provided");

    let mut input_dir = "generated_files/input";
    if !input.is_empty() {
        input_dir = input;
    }
    let contents = get_file_contents_from_dir(input_dir);
    println!("contents.len() {}", contents.len());

    let split_content: Vec<(String, Vec<String>)> = contents
        .iter()
        .map(|(filename, content_string)| {
            let mut splitted = content_string
                .split("-------------")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            splitted.remove(0);
            (filename.to_string(), splitted)
        })
        .collect();
    println!("split_content: {:?}", split_content[1]);
}

pub fn get_file_contents_from_dir(dir: &str) -> Vec<(String, String)> {
    let filepaths = get_file_paths_from_dir(dir);
    let mut file_id_contents: Vec<(String, String)> = Vec::new();
    for filepath in filepaths {
        let file_path_str = filepath
            .file_name()
            .expect("Invalid file path! (likely terminating in /..)")
            .to_str()
            .expect("File name could not be converted from an OsStr to a String!")
            .to_string();
        file_id_contents.push((
            file_path_str.clone(),
            read_to_string(filepath).unwrap_or_else(|_| {
                panic!(
                    "{} cannot be read as an UTF-8 file! Please ensure it is in the UTF-8 format.",
                    file_path_str
                )
            }),
        ))
    }
    file_id_contents
}

/// Returns a list of paths to files (not subdirectories) that are in a directory
pub fn get_file_paths_from_dir(dir: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in read_dir(dir).unwrap_or_else(|_| {
        panic!(
            "Path {} doesn't exist, no permissions, or is not a directory!",
            dir
        )
    }) {
        let path = entry.expect("Cannot parse this directory entry!").path();
        if !path.is_dir() {
            paths.push(path);
        }
    }
    paths
}
