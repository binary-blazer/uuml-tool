#!/bin/bash

# If you want to update this script, please make sure your code remains clean and readable.
# The mission is to keep this file small and as simple as possible to make it easy to maintain.

# Check if the script is run as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

# Remove the executable from /usr/local/bin
rm -f /usr/local/bin/uuml

echo "Uninstallation complete. 'uuml' has been removed from your system."
