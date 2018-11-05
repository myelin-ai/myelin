#requires -version 5.1

Param (
    [Switch]$noWebpack = $false,
    [Switch]$release = $false
    [Switch]$help = $false
 )

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ($help) {
    Write-Output "Usage: $PSScriptRoot [flags]"
    Write-Output ""
    Write-Output "Supported flags:"
    Write-Output "--no-webpack    Builds without webpack"
    Write-Output "--release       Builds in release mode"
    Write-Output "--help          Prints this help"
    exit
}

$releaseFlag = if ($release) {
    $releaseFlag = "--release"
} else {
    ""
}

$crateDir = "$PSScriptRoot/../"
$targetDir = "$crateDir/out"

Set-Location -Path "$crateDir"

$applicationName = "myelin_visualization_client"

cargo build --target wasm32-unknown-unknown $release_flag

Remove-Item -Recurse -Path "$targetDir"
New-Item -ItemType directory -Path "$targetDir"
wasm-bindgen ../target/wasm32-unknown-unknown/debug/$applicationName.wasm `
             --out-dir "$targetDir"

if (!($noWebpack)) {
    yarn
    yarn webpack
}
