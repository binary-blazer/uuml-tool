# UUML Tool

## Description

The ÜÜML Tool is a command-line utility designed to replace German Umlauts in HTML files with their corresponding HTML entities. This tool is particularly useful for web developers who need to ensure that their HTML files are correctly encoded for different browsers and platforms.

### Features

- Replace Umlauts (ä, ö, ü, Ä, Ö, Ü, ß) with their HTML entities (&auml;, &ouml;, &uuml;, &Auml;, &Ouml;, &Uuml;, &szlig;)
- Process individual HTML files or entire directories containing HTML files
- Simple and easy-to-use command-line interface

## Usage

To use the ÜÜML Tool, you can specify either a single HTML file or a directory containing HTML files. The tool will process the specified file or all HTML files in the specified directory, replacing Umlauts with their corresponding HTML entities.

### Examples

Replace Umlauts in a single HTML file:

```sh
$ uuml -f path/to/file.html
```

Replace Umlauts in all HTML files in a directory:

```sh
$ uuml -d path/to/directory
```

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (required to build the tool)

### Building from Source

To build the ÜÜML Tool from source, follow these steps:

1. Clone the repository:

```sh
$ git clone https://github.com/binary-blazer/uuml-tool.git
$ cd uuml-tool
```

2. Build the tool using Cargo:

```sh
$ cargo build --release
```

3. The compiled binary will be located in the `target/release` directory.

### Installing on Different Operating Systems

#### Windows

1. Download the precompiled binary for Windows from the [releases page](https://github.com/binary-blazer/uuml-tool/releases).
2. Extract the downloaded ZIP file.
3. Move the extracted binary to a directory in your system's `PATH`.

#### macOS

1. Download the precompiled binary for macOS from the [releases page](https://github.com/binary-blazer/uuml-tool/releases).
2. Extract the downloaded ZIP file.
3. Move the extracted binary to a directory in your system's `PATH`.

#### Linux

1. Download the precompiled binary for Linux from the [releases page](https://github.com/binary-blazer/uuml-tool/releases).
2. Extract the downloaded TAR file.
3. Move the extracted binary to a directory in your system's `PATH`.

Alternatively, you can build the tool from source as described in the "Building from Source" section.
