// fn main() {
//     println!("Hello, world!");
// }





use clap::{Arg, Command};
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("dir_compare")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Compares files in two directories")
        .arg(
            Arg::new("dir1")
                .about("First directory to compare")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dir2")
                .about("Second directory to compare")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .about("Output file path"),
        )
        .get_matches();

    let dir1 = matches.value_of("dir1").unwrap();
    let dir2 = matches.value_of("dir2").unwrap();
    let output = matches.value_of("output");

    let diff_files = compare_directories(dir1, dir2);

    if let Some(output_path) = output {
        fs::write(output_path, diff_files.join("\n")).expect("Unable to write to file");
    } else {
        for file in diff_files {
            println!("{}", file);
        }
    }
}

fn compare_directories(dir1: &str, dir2: &str) -> Vec<String> {
    let mut diff_files = Vec::new();
    let files1 = get_files(dir1);
    let files2 = get_files(dir2);

    let all_files: HashSet<_> = files1.keys().chain(files2.keys()).collect();

    for file in all_files {
        let path1 = files1.get(file);
        let path2 = files2.get(file);

        if let (Some(path1), Some(path2)) = (path1, path2) {
            if !compare_files(path1, path2) {
                diff_files.push(file.clone());
            }
        } else {
            diff_files.push(file.clone());
        }
    }

    diff_files
}

fn get_files(dir: &str) -> std::collections::HashMap<String, String> {
    let mut files = std::collections::HashMap::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let relative_path = entry.path().strip_prefix(dir).unwrap().to_string_lossy().to_string();
            files.insert(relative_path, entry.path().to_string_lossy().to_string());
        }
    }
    files
}

fn compare_files(file1: &str, file2: &str) -> bool {
    let metadata1 = fs::metadata(file1).expect("Unable to read metadata");
    let metadata2 = fs::metadata(file2).expect("Unable to read metadata");

    if metadata1.len() != metadata2.len() {
        return false;
    }

    let file1 = fs::File::open(file1).expect("Unable to open file");
    let file2 = fs::File::open(file2).expect("Unable to open file");

    let reader1 = io::BufReader::new(file1);
    let reader2 = io::BufReader::new(file2);

    let lines1: Vec<_> = reader1.lines().collect();
    let lines2: Vec<_> = reader2.lines().collect();

    if lines1.len() != lines2.len() {
        return false;
    }

    for (line1, line2) in lines1.iter().zip(lines2.iter()) {
        if line1.as_ref().unwrap() != line2.as_ref().unwrap() {
            return false;
        }
    }

    true
}