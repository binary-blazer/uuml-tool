# If you want to update this script, please make sure your code remains clean and readable.
# The mission is to keep this file small and as simple as possible to make it easy to maintain.

# Check if the script is run as administrator
If (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "Please run this script as an administrator."
    Exit 1
}

# Remove the executable from the PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine) -replace ";C:\\Program Files\\uuml", ""
[System.Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::Machine)

# Remove the executable
Remove-Item -Path "C:\Program Files\uuml\uuml.exe" -Force

Write-Output "Uninstallation complete. 'uuml' has been removed from your PATH."
