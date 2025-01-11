#!/bin/bash

# Check if the script is run as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

# Get the latest release download URL from GitHub
latest_release_url=$(curl -s https://api.github.com/repos/binary-blazer/uuml-tool/releases/latest | grep "browser_download_url.*linux-gnu" | cut -d : -f 2,3 | tr -d \")

# Download the latest release
curl -L -o /usr/local/bin/uuml $latest_release_url

# Make the file executable
chmod +x /usr/local/bin/uuml

echo "Installation complete. 'uuml' is now available in your PATH."
