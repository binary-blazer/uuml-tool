use clap::{Arg, Command};
use std::fs;

fn main() {
    let matches = Command::new("uuml")
        .version("1.2")
        .author("Jonas F. Franke <@jonasfranke@sdevs.org>")
        .about("Replaces Umlauts with their HTML entities")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the HTML file to use")
                .num_args(1),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("Sets the directory containing HTML files to use")
                .num_args(1),
        )
        .get_matches();

    if let Some(file) = matches.get_one::<String>("file") {
        process_file(file);
    } else if let Some(directory) = matches.get_one::<String>("directory") {
        process_directory(directory);
    } else {
        process_directory(".");
    }
}

fn process_file(file: &str) {
    let content = fs::read_to_string(file).expect("Could not read file");
    let processed_content = replace_umlauts(&content);
    fs::write(file, processed_content).expect("Could not write file");
    println!("Processed file: {}", file);
}

fn process_directory(directory: &str) {
    for entry in fs::read_dir(directory).expect("Could not read directory") {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("html") {
            process_file(path.to_str().expect("Could not convert path to str"));
        }
    }
    println!("Processed directory: {}", directory);
}

fn replace_umlauts(content: &str) -> String {
    let mut result = String::new();
    let mut in_comment = false;

    for line in content.lines() {
        if line.contains("<!--") {
            in_comment = true;
        }
        if line.contains("-->") {
            in_comment = false;
        }

        if in_comment {
            result.push_str(line);
        } else {
            result.push_str(
                &line
                    .replace("ä", "&auml;")
                    .replace("ö", "&ouml;")
                    .replace("ü", "&uuml;")
                    .replace("Ä", "&Auml;")
                    .replace("Ö", "&Ouml;")
                    .replace("Ü", "&Uuml;")
                    .replace("ß", "&szlig;"),
            );
        }
        result.push('\n');
    }

    result
}