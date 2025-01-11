use clap::{Arg, Command};
use std::fs;
use std::process::Command as ProcessCommand;
use std::env;

fn main() {
    let matches = Command::new("uuml")
        .version("1.1")
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
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .help("Updates the CLI to the latest version")
                .num_args(0),
        )
        .get_matches();

    if matches.contains_id("update") {
        update_cli();
    } else if let Some(file) = matches.get_one::<String>("file") {
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

fn update_cli() {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let current_exe_path = current_exe.to_str().expect("Failed to convert path to str");

    let latest_release_url = if cfg!(target_os = "windows") {
        "https://api.github.com/repos/binary-blazer/uuml-tool/releases/latest"
    } else {
        "https://api.github.com/repos/binary-blazer/uuml-tool/releases/latest"
    };

    let output = ProcessCommand::new("curl")
        .arg("-s")
        .arg(latest_release_url)
        .output()
        .expect("Failed to execute curl command");

    let response = String::from_utf8_lossy(&output.stdout);
    let download_url = if cfg!(target_os = "windows") {
        response
            .lines()
            .find(|line| line.contains("browser_download_url") && line.contains("uuml-win.exe"))
            .expect("Failed to find download URL")
            .split('"')
            .nth(3)
            .expect("Failed to extract download URL")
    } else {
        response
            .lines()
            .find(|line| line.contains("browser_download_url") && line.contains("uuml-linux"))
            .expect("Failed to find download URL")
            .split('"')
            .nth(3)
            .expect("Failed to extract download URL")
    };

    let temp_path = if cfg!(target_os = "windows") {
        "C:\\Program Files\\uuml\\temp_uuml.exe"
    } else {
        "/usr/local/bin/temp_uuml"
    };

    ProcessCommand::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(temp_path)
        .arg(download_url)
        .status()
        .expect("Failed to download latest version");

    if cfg!(target_os = "windows") {
        ProcessCommand::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Move-Item -Force -Path '{}' -Destination '{}'",
                temp_path, current_exe_path
            ))
            .status()
            .expect("Failed to replace executable");
    } else {
        ProcessCommand::new("sh")
            .arg("-c")
            .arg(format!("mv {} {}", temp_path, current_exe_path))
            .status()
            .expect("Failed to replace executable");
    }

    println!("Update complete. Please restart the CLI.");
}