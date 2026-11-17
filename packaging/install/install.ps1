# Installs the grimoire command-line tool on Windows.
#
# This file is generated for each release by `cargo xtask release channels`
# from the template packaging/install/install.ps1, which fills in the version
# and the SHA-256 of each archive. See docs/release.md and
# docs/adr/0010-language-policy-exceptions.md.
#
# It downloads one archive over HTTPS, checks its SHA-256 against the value
# embedded below before extracting it, and puts grimoire.exe into one
# directory. It never changes PATH or the registry. It sends nothing anywhere:
# no telemetry, no receipts, no tokens, no updater.
#
# Usage: powershell -ExecutionPolicy Bypass -File .\install.ps1 [-InstallDir DIR]
# (see -Help). Run it with -File: on failure it exits with status 1.

[CmdletBinding()]
param(
    [string] $InstallDir = '',
    [switch] $Help
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

# Filled in by `cargo xtask release channels`. Empty means "not built".
$Version = '@@VERSION@@'
$Sha256 = @{
    'x86_64-pc-windows-msvc'  = '@@SHA256:x86_64-pc-windows-msvc@@'
    'aarch64-pc-windows-msvc' = '@@SHA256:aarch64-pc-windows-msvc@@'
}

$RepoUrl = 'https://github.com/jabbott-iii/Grimoire'
$Usage = @"
Install grimoire $Version.

Usage: powershell -ExecutionPolicy Bypass -File .\install.ps1 [-InstallDir DIR]

Options:
  -InstallDir DIR  Install into DIR, creating it if needed.
  -Help            Show this help.

Environment:
  GRIMOIRE_INSTALL_DIR   Install directory when -InstallDir is not given.
                         Default: %LOCALAPPDATA%\Programs\grimoire\bin
  GRIMOIRE_DOWNLOAD_URL  Base URL to download the archive from. Default:
                         $RepoUrl/releases/download/v$Version
                         It must start with https://. Plain http:// is allowed
                         only for 127.0.0.1 and localhost, for testing.

The archive's SHA-256 is checked against the value embedded in this script
before anything is extracted. PATH and the registry are never changed.
"@

# Temporary files, removed on any exit.
$script:TempDir = $null
$script:Staged = $null
# While swapping: the destination and the old binary's backup, restored if the swap fails.
$script:Dest = $null
$script:Backup = $null

function Get-Target {
    [OutputType([string])]
    param()
    $arch = ''
    try {
        $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
    } catch {
        # Not available on every .NET Framework; fall back to the variables. In a 32-bit
        # PowerShell on 64-bit Windows (WOW64), PROCESSOR_ARCHITECTURE says x86 and
        # PROCESSOR_ARCHITEW6432 holds the real architecture.
        $arch = $env:PROCESSOR_ARCHITEW6432
        if (-not $arch) { $arch = $env:PROCESSOR_ARCHITECTURE }
    }
    # dist's installer notes that OSArchitecture may report X64 on Arm64
    # before PowerShell 7.3 or Windows 11 22H2 (UNVERIFIED). That would only
    # pick the x64 build.
    switch ($arch) {
        { $_ -in 'X64', 'AMD64' } { return 'x86_64-pc-windows-msvc' }
        'Arm64' { return 'aarch64-pc-windows-msvc' }
        default { throw "there is no grimoire build for Windows on '$arch'" }
    }
}

# Accepts https://, and plain http:// only for the loopback host. The
# character set leaves out '@', so no user info can hide the real host.
function Test-DownloadUrl {
    [OutputType([bool])]
    param([string] $Url)
    if ($Url -cmatch '[^A-Za-z0-9._~:/%+-]') { return $false }
    if ($Url -cmatch '^https://.') { return $true }
    return ($Url -cmatch '^http://(127\.0\.0\.1|localhost)(:[0-9]+)?(/.*)?$')
}

# True if the program at Path runs and exits 0 with --version.
function Test-Executable {
    [OutputType([bool])]
    param([string] $Path)
    # Keep output on stderr from stopping the script.
    $ErrorActionPreference = 'Continue'
    try {
        & $Path --version *> $null
        return ($LASTEXITCODE -eq 0)
    } catch {
        return $false
    }
}

function Install-Grimoire {
    param([string] $Dir)

    if ($Version.Contains('@')) {
        throw 'this is the unrendered template; use the install.ps1 attached to a Grimoire release'
    }

    if (-not $Dir) { $Dir = $env:GRIMOIRE_INSTALL_DIR }
    if (-not $Dir) {
        if (-not $env:LOCALAPPDATA) {
            throw 'LOCALAPPDATA is not set; pass -InstallDir or set GRIMOIRE_INSTALL_DIR'
        }
        $Dir = Join-Path -Path $env:LOCALAPPDATA -ChildPath 'Programs\grimoire\bin'
    }
    # .NET resolves relative paths against the process directory, not $PWD.
    $Dir = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Dir)

    $target = Get-Target
    $expected = $Sha256[$target]
    if (-not $expected) {
        throw "this script has no SHA-256 for $target, so it cannot verify the download"
    }
    if ($expected -cnotmatch '^[0-9a-f]{64}$') {
        throw "the SHA-256 embedded for $target is malformed"
    }

    $base = $env:GRIMOIRE_DOWNLOAD_URL
    if (-not $base) { $base = "$RepoUrl/releases/download/v$Version" }
    if (-not (Test-DownloadUrl -Url $base)) {
        throw 'GRIMOIRE_DOWNLOAD_URL must start with https:// (plain http:// is allowed only for 127.0.0.1 and localhost) and contain only letters, digits and ._~:/%+-'
    }
    $name = "agent-grimoire-$Version-$target.zip"
    $url = $base.TrimEnd('/') + '/' + $name

    # Windows PowerShell 5.1 may not offer TLS 1.2 by default.
    if ($PSVersionTable.PSVersion.Major -lt 6) {
        [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
    }

    $script:TempDir = Join-Path -Path ([System.IO.Path]::GetTempPath()) -ChildPath ('grimoire-install-' + [System.Guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $script:TempDir | Out-Null
    $archive = Join-Path -Path $script:TempDir -ChildPath $name

    Write-Output "downloading $url"
    try {
        Invoke-WebRequest -Uri $url -OutFile $archive -UseBasicParsing
    } catch {
        throw "could not download ${url}: $($_.Exception.Message)"
    }

    $actual = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -cne $expected) {
        throw "SHA-256 mismatch for ${name}: expected $expected, got $actual"
    }
    Write-Output "verified SHA-256 $actual"

    # Only now, with the archive verified, extract it. The archive is flat.
    $extracted = Join-Path -Path $script:TempDir -ChildPath 'x'
    Expand-Archive -LiteralPath $archive -DestinationPath $extracted
    $bin = Join-Path -Path $extracted -ChildPath 'grimoire.exe'
    if (-not (Test-Path -LiteralPath $bin -PathType Leaf)) {
        throw "$name has no grimoire.exe"
    }

    New-Item -ItemType Directory -Path $Dir -Force | Out-Null
    $dest = Join-Path -Path $Dir -ChildPath 'grimoire.exe'
    if (Test-Path -LiteralPath $dest -PathType Container) {
        throw "$dest is a directory"
    }

    # Stage a copy next to the destination, check that it runs, then swap it
    # in, so grimoire.exe is never missing or half-written.
    $script:Staged = Join-Path -Path $Dir -ChildPath ('.grimoire-' + [System.Guid]::NewGuid().ToString('N') + '.exe')
    Copy-Item -LiteralPath $bin -Destination $script:Staged
    if (-not (Test-Executable -Path $script:Staged)) {
        throw 'the downloaded grimoire.exe does not run on this machine; nothing was replaced'
    }
    if (Test-Path -LiteralPath $dest -PathType Leaf) {
        # With a backup, a failed ReplaceFile leaves the old binary either in place or
        # under the backup name, and the finally block below puts it back.
        $script:Dest = $dest
        $script:Backup = Join-Path -Path $Dir -ChildPath ('.grimoire-' + [System.Guid]::NewGuid().ToString('N') + '.old.exe')
        [System.IO.File]::Replace($script:Staged, $dest, $script:Backup)
        $script:Staged = $null
        # A running grimoire.exe can be renamed but not deleted; its backup then stays.
        Remove-Item -LiteralPath $script:Backup -Force -ErrorAction SilentlyContinue
        $script:Backup = $null
    } else {
        [System.IO.File]::Move($script:Staged, $dest)
    }
    $script:Staged = $null

    Write-Output "installed grimoire $Version to $dest"
    $onPath = $false
    foreach ($entry in ($env:Path -split ';')) {
        if ($entry -and ($entry.TrimEnd('\') -eq $Dir.TrimEnd('\'))) { $onPath = $true }
    }
    if (-not $onPath) {
        Write-Output "$Dir is not on your PATH. To run grimoire by name, add it to your user Path environment variable and open a new terminal. For this session only:"
        Write-Output "  `$env:Path = `"$Dir;`$env:Path`""
    }
}

if ($Help) {
    Write-Output $Usage
    exit 0
}

# $IsWindows exists only in PowerShell 6 and later; 5.1 runs only on Windows.
$isWindowsVar = Get-Variable -Name 'IsWindows' -ValueOnly -ErrorAction SilentlyContinue
if (($null -ne $isWindowsVar) -and (-not $isWindowsVar)) {
    [Console]::Error.WriteLine('error: install.ps1 is for Windows; on Linux and macOS, use install.sh instead')
    exit 1
}

$exitCode = 0
try {
    Install-Grimoire -Dir $InstallDir
} catch {
    $message = ($_.Exception.Message -replace '\s+', ' ').Trim()
    [Console]::Error.WriteLine("error: $message")
    $exitCode = 1
} finally {
    # A swap that failed halfway: put the old binary back before anything is deleted.
    if ($script:Backup -and (Test-Path -LiteralPath $script:Backup -PathType Leaf) -and
        -not (Test-Path -LiteralPath $script:Dest)) {
        Move-Item -LiteralPath $script:Backup -Destination $script:Dest -ErrorAction SilentlyContinue
    }
    if ($script:Staged) { Remove-Item -LiteralPath $script:Staged -Force -ErrorAction SilentlyContinue }
    if ($script:TempDir) { Remove-Item -LiteralPath $script:TempDir -Recurse -Force -ErrorAction SilentlyContinue }
}
exit $exitCode
