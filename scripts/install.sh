#!/bin/bash

# If you want to update this script, please make sure your code remains clean and readable.
# The mission is to keep this file small and as simple as possible to make it easy to maintain.

# Check if the script is run as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

# Get the latest release download URL from GitHub
latest_release_url=$(curl -s https://api.github.com/repos/binary-blazer/uuml-tool/releases/latest | grep "browser_download_url.*uuml-linux" | cut -d : -f 2,3 | tr -d \")

# Create the directory if it doesn't exist
destination_dir="/usr/local/bin"
if [ ! -d "$destination_dir" ]; then
  mkdir -p "$destination_dir"
fi

# Download the latest release
curl -L -o /usr/local/bin/uuml $latest_release_url

# Make the file executable
chmod +x /usr/local/bin/uuml

echo "Installation complete. 'uuml' is now available in your PATH."
echo "Reload/Restart your console and run 'uuml --help' to get started."