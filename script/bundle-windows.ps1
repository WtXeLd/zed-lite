[CmdletBinding()]
Param(
    [Parameter()][Alias('i')][switch]$Install,
    [Parameter()][Alias('h')][switch]$Help,
    [Parameter()][Alias('a')][string]$Architecture
)

. "$PSScriptRoot/lib/workspace.ps1"

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Help) {
    Write-Output "Usage: bundle-windows.ps1 [-Architecture x86_64|aarch64] [-Install]"
    Write-Output "Build the Zed Lite Windows package."
    exit 0
}

$OSArchitecture = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    "X64" { "x86_64" }
    "Arm64" { "aarch64" }
    default { throw "Unsupported architecture" }
}

$Architecture = if ($Architecture) {
    $Architecture
} else {
    $OSArchitecture
}

$target = "$Architecture-pc-windows-msvc"

function Get-VSArch {
    param([string]$Arch)

    switch ($Arch) {
        "x86_64" { "amd64" }
        "aarch64" { "arm64" }
        default { throw "Unsupported architecture $Arch" }
    }
}

function Enter-VsDevShell {
    $vsDevShell = "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1"
    if (Test-Path $vsDevShell) {
        Push-Location
        & $vsDevShell -Arch (Get-VSArch -Arch $Architecture) -HostArch (Get-VSArch -Arch $OSArchitecture)
        Pop-Location
    }
}

function GenerateLicenses {
    . $PSScriptRoot/generate-licenses.ps1
}

function BuildZedLite {
    rustup target add $target
    cargo build --release --no-default-features --features zed/lite,cli/lite --package zed --package cli --target $target
}

function PackageZedLite {
    $targetRoot = $env:CARGO_TARGET_DIR ? $env:CARGO_TARGET_DIR : "$env:ZED_WORKSPACE\target"
    $cargoOutDir = "$targetRoot\$target\release"
    $packageRoot = "$targetRoot\zed-lite-windows-$Architecture"

    if (Test-Path $packageRoot) {
        Remove-Item -Path $packageRoot -Recurse -Force
    }

    New-Item -Path $packageRoot -ItemType Directory -Force | Out-Null
    New-Item -Path "$packageRoot\bin" -ItemType Directory -Force | Out-Null

    Copy-Item -Path "$cargoOutDir\zed.exe" -Destination "$packageRoot\Zed Lite.exe" -Force
    Copy-Item -Path "$cargoOutDir\cli.exe" -Destination "$packageRoot\bin\zed.exe" -Force

    if (Test-Path "$cargoOutDir\conpty.dll") {
        Copy-Item -Path "$cargoOutDir\conpty.dll" -Destination "$packageRoot\conpty.dll" -Force
    }

    if (Test-Path "$cargoOutDir\OpenConsole.exe") {
        Copy-Item -Path "$cargoOutDir\OpenConsole.exe" -Destination "$packageRoot\OpenConsole.exe" -Force
    }

    Copy-Item -Path "$env:ZED_WORKSPACE\assets\licenses.md" -Destination "$packageRoot\licenses.md" -Force

    $archive = "$targetRoot\zed-lite-windows-$Architecture.zip"
    if (Test-Path $archive) {
        Remove-Item -Path $archive -Force
    }

    Compress-Archive -Path "$packageRoot\*" -DestinationPath $archive -Force
    Write-Output "Created $archive"

    if ($Install) {
        Start-Process -FilePath "$packageRoot\Zed Lite.exe"
    }
}

ParseZedWorkspace
Enter-VsDevShell
GenerateLicenses
BuildZedLite
PackageZedLite
