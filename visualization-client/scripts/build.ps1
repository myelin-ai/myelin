#requires -version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$applicationName = "myelin_visualization_client"
$targetDir = "visualization-client/out"

cargo build -p myelin-visualization-client --target wasm32-unknown-unknown

Remove-Item -Recurse -Path $targetDir
New-Item -ItemType directory -Path $targetDir
wasm-bindgen target/wasm32-unknown-unknown/debug/$applicationName.wasm --out-dir $targetDir

Set-Location -Path visualization-client
yarn
yarn webpack
Set-Location -Path ..
