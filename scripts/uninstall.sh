#!/bin/bash

# Check if the script is run as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

# Remove the executable from /usr/local/bin
rm -f /usr/local/bin/uuml

echo "Uninstallation complete. 'uuml' has been removed from your system."
