# Windows MSI Packaging

This directory contains WiX configuration and assets to build a fast, clean MSI installer.

## Quick Steps (Windows)
1. Build the release binary:
   - `cargo build --release`
2. Stage the output:
   - Copy `target/release/ncom_audax.exe` to `installer/staging/NCOM_Audax.exe`.
   - Copy assets to `installer/staging/assets/`.
3. Generate GUIDs for Product.wxs:
   - Update the `UpgradeCode` and each `Component` Guid.
4. Build MSI with WiX:
   - `wix build installer/wix/Product.wxs -ext WixToolset.UI.wixext -o dist/NCOM-Audax-v0.1.msi`

## Notes
- Update `installer/wix/wix.toml` with actual product/upgrade codes.
- Replace `installer/assets/app_icon.ico`, `banner.bmp`, and `dialog.bmp` with official art.
- The MSI uses WixUI_InstallDir for a clean install wizard with a custom install path.
