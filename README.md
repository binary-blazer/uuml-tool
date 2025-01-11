# UUML Tool

The UUML Tool is a command-line utility designed to replace German Umlauts in HTML files with their corresponding HTML entities. This tool is particularly useful for web developers who need to ensure that their HTML files are correctly encoded for different browsers and platforms.

## Features

- Replace Umlauts (ä, ö, ü, Ä, Ö, Ü, ß) with their HTML entities (&auml;, &ouml;, &uuml;, &Auml;, &Ouml;, &Uuml;, &szlig;)
- Also replaces the € symbol with its HTML entity (&euro;)
- Process individual HTML files or entire directories containing HTML files
- Simple and easy-to-use command-line interface
- Platform support (Windows, Linux), no macOS support yet

## Usage

To use the UUML Tool, you can specify either a single HTML file or a directory containing HTML files. The tool will process the specified file or all HTML files in the specified directory, replacing Umlauts with their corresponding HTML entities.

### Examples

Replace Umlauts in a single HTML file:

```sh
uuml -f path/to/file.html
```

Replace Umlauts in all HTML files in a directory:

```sh
uuml -d path/to/directory
```

For the more lazy people, you can also simply call ```uuml``` without any arguments. This will replace Umlauts in all HTML files in the current directory.

```sh
uuml
```

### Update

To update the UUML Tool to the latest version, you can use the `update` command:

```sh
uuml --update
```

## Installation

The UUML Tool is distributed as a standalone executable file for Windows and Linux. You can install it by the bellow installation files.

### Windows

There is a quick installation script for Windows. Just run the following command in a PowerShell window:

```sh
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/binary-blazer/uuml-tool/refs/heads/master/scripts/install.ps1'))
```

### Linux

There is a quick installation script for Linux. Just run the following command in a terminal window:

```sh
curl -sSL https://raw.githubusercontent.com/binary-blazer/uuml-tool/refs/heads/master/scripts/install.sh | bash
```

## Uninstallation

To uninstall the UUML Tool, you can simply delete the executable file from your system. If you installed the tool using the installation scripts, you can run the following commands to uninstall it.

### Windows

Run the following command in a PowerShell window:

```sh
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/binary-blazer/uuml-tool/refs/heads/master/scripts/uninstall.ps1'))
```

### Linux

Run the following command in a terminal window:

```sh
curl -sSL https://raw.githubusercontent.com/binary-blazer/uuml-tool/refs/heads/master/scripts/uninstall.ps1 | bash
```
