# Check if the script is run as administrator
If (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "Please run this script as an administrator."
    Exit 1
}

# Get the latest release download URL from GitHub
$latestReleaseUrl = (Invoke-RestMethod -Uri "https://api.github.com/repos/binary-blazer/uuml-tool/releases/latest").assets | Where-Object { $_.name -like "*windows-gnu*" } | Select-Object -ExpandProperty browser_download_url

# Download the latest release
$destinationPath = "C:\Program Files\uuml\uuml.exe"
Invoke-WebRequest -Uri $latestReleaseUrl -OutFile $destinationPath

# Add the executable to the PATH
$env:Path += ";C:\Program Files\uuml"
[System.Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::Machine)

Write-Output "Installation complete. 'uuml' is now available in your PATH."
