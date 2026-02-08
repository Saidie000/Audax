Param(
    [string]$Configuration = "Release"
)

$Root = Resolve-Path "$PSScriptRoot/../.."
$Staging = Join-Path $Root "installer/staging"
$Assets = Join-Path $Staging "assets"

Write-Host "Building NCOM Audax ($Configuration)"

cargo build --release

if (!(Test-Path $Staging)) {
    New-Item -ItemType Directory -Path $Staging | Out-Null
}
if (!(Test-Path $Assets)) {
    New-Item -ItemType Directory -Path $Assets | Out-Null
}

Copy-Item "$Root/target/release/ncom_audax.exe" "$Staging/NCOM_Audax.exe" -Force
Copy-Item "$Root/assets/*" $Assets -Recurse -Force

Write-Host "Building MSI with WiX..."
wix build "$Root/installer/wix/Product.wxs" -ext WixToolset.UI.wixext -o "$Root/dist/NCOM-Audax-v0.1.msi"
