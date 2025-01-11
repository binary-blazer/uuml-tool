# If you want to update this script, please make sure your code remains clean and readable.
# The mission is to keep this file small and as simple as possible to make it easy to maintain.

# Check if the script is run as administrator
If (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "Please run this script as an administrator."
    Exit 1
}

# Get the latest release download URL from GitHub
$latestReleaseUrl = (Invoke-RestMethod -Uri "https://api.github.com/repos/binary-blazer/uuml-tool/releases/latest").assets | Where-Object { $_.name -eq "uuml-win.exe" } | Select-Object -ExpandProperty browser_download_url

# Create the directory if it doesn't exist
$destinationDir = "C:\Program Files\uuml"
If (-Not (Test-Path -Path $destinationDir)) {
    New-Item -ItemType Directory -Path $destinationDir
}

# Download the latest release
$destinationPath = "$destinationDir\uuml.exe"
Invoke-WebRequest -Uri $latestReleaseUrl -OutFile $destinationPath

# Add the executable to the PATH
$env:Path += ";C:\Program Files\uuml"
[System.Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::Machine)

Write-Output "Installation complete. 'uuml' is now available in your PATH."
Write-Output "Reload/Restart your console and run 'uuml --help' to get started."