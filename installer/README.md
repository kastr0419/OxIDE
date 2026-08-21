ALLoIDE Windows installer (Inno Setup)

This folder contains the Inno Setup script and helper to build a full Windows installer that bundles ALLoIDE and options to install the Rust toolchain and avrdude.

Files:
- oxide_setup.iss  : Inno Setup 6 script for the installer
- build_installer.ps1 : PowerShell helper to build the installer (will build the binary, download rustup-init.exe and attempt to download avrdude.exe, then run ISCC)
- tools/           : Place rustup-init.exe and avrdude.exe here (build_installer.ps1 can download rustup-init automatically)
- output/          : Installer output (ignored by git)

How to build
1. Ensure you have Inno Setup 6 installed (ISCC.exe available).
2. Ensure cargo is available to build the release binary, or place D:\OxIDE\target\release\oxide.exe manually.
3. Open an elevated PowerShell and run:
   .\build_installer.ps1

Notes
- Current implementation identifiers remain unchanged: `oxide.exe` is the binary, `OxIDE_Setup_0.1.0.exe` is the installer output, `OxIDE` is the installer/shortcut display name, and `{pf}\OxIDE` is the install directory.
- The installer will attempt to run rustup-init.exe silently during installation. If rustup is already installed on the target machine, the installer will skip rustup-init.
- The installer adds {pf}\OxIDE to the system PATH and creates shortcuts.
- ESP32 (xtensa) toolchain is not bundled and requires the espup tool (user must install separately).

日本語:
インストーラを作る手順:
1. Inno Setup 6 をインストール
2. 管理者権限の PowerShell で build_installer.ps1 を実行
3. 出力は installer\\output\\ に作成されます
