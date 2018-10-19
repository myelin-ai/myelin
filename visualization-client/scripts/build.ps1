#requires -version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$crateDir = "$PSScriptRoot/../"
$targetDir = "$crateDir/out"

Set-Location -Path "$crateDir"

$applicationName = "myelin_visualization_client"

cargo build --target wasm32-unknown-unknown

Remove-Item -Recurse -Path "$targetDir"
New-Item -ItemType directory -Path "$targetDir"
wasm-bindgen ../target/wasm32-unknown-unknown/debug/$applicationName.wasm `
             --out-dir "$targetDir"

yarn
yarn webpack
