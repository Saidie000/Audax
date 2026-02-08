# Audax
![NCOM Audax app icon](assets/icons/app_icon.svg)

Audax is a fully free drawing app.

## Install (Windows MSI)
Use the official MSI package from GitHub Releases:

```cmd
curl -L -o "%TEMP%\\NCOM-Audax-v0.1.msi" https://github.com/OWNER/Audax/releases/latest/download/NCOM-Audax-v0.1.msi && start /wait msiexec.exe /i "%TEMP%\\NCOM-Audax-v0.1.msi" /qn
```

Or with PowerShell:

```powershell
Invoke-WebRequest -Uri "https://github.com/OWNER/Audax/releases/latest/download/NCOM-Audax-v0.1.msi" -OutFile "$env:TEMP\\NCOM-Audax-v0.1.msi"; Start-Process "msiexec.exe" -ArgumentList "/i `"$env:TEMP\\NCOM-Audax-v0.1.msi`" /qn" -Wait
```

Replace `OWNER` with your GitHub organization/user name.

## Packaging & Release (NCOM Audax v0.1)
- MSI packaging and GitHub Release automation are scaffolded in `src/package.rs` and should be wired into a CI pipeline or build tooling.
- The app icon asset should be placed at `assets/icons/app_icon.png` (see placeholder notes in `assets/icons/README.md`).
- WiX installer scaffolding lives in `installer/` with scripts for staging and MSI builds.

## Documentation Site
- Static documentation site lives in `docs/` and is intended for Docs.Audax.
- Deploy `docs/` as a static site (GitHub Pages, S3, or Azure Static Web Apps).

## Input
- Advanced palm rejection is supported via `InputManager` configuration in `src/input/palm_rejection.rs`.
