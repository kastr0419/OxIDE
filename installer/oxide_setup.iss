[Setup]
AppName=OxIDE
AppVersion=0.1.0
AppPublisher=kastr0419
AppPublisherURL=https://github.com/kastr0419/OxIDE
DefaultDirName={autopf}\OxIDE
DefaultGroupName=OxIDE
OutputDir=D:\OxIDE\installer\output
OutputBaseFilename=OxIDE_Setup_0.1.0
;SetupIconFile=D:\OxIDE\assets\oxide.ico
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ChangesEnvironment=yes
LicenseFile=D:\OxIDE\LICENSE-MIT

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\\Japanese.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "デスクトップにショートカットを作成"; GroupDescription: "追加タスク:"; Flags: unchecked

[Files]
; Main executable
Source: "D:\OxIDE\target\release\oxide.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\OxIDE\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\OxIDE\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\OxIDE\NOTICE"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\OxIDE\OSS_LICENSE_AUDIT.md"; DestDir: "{app}"; Flags: ignoreversion

; Rustup installer (will be copied to temporary folder and removed after install)
Source: "D:\OxIDE\installer\tools\rustup-init.exe"; DestDir: "{tmp}"; Flags: deleteafterinstall

; avrdude
Source: "D:\OxIDE\installer\tools\avrdude.exe"; DestDir: "{app}\\tools"; Flags: ignoreversion

[Icons]
Name: "{group}\OxIDE"; Filename: "{app}\\oxide.exe"
Name: "{group}\OxIDEのアンインストール"; Filename: "{uninstallexe}"
Name: "{commondesktop}\OxIDE"; Filename: "{app}\\oxide.exe"; Tasks: desktopicon

[Run]
; Install Rust (only run rustup-init if rustup is not already installed)
Filename: "{tmp}\\rustup-init.exe"; Parameters: "-y --default-toolchain stable --no-modify-path"; StatusMsg: "Rustをインストール中..."; Flags: waituntilterminated; Check: not FileExists(ExpandConstant('{%USERPROFILE}\\.cargo\\bin\\rustup.exe'))

; Ensure rustup target additions (run if rustup is available)
Filename: "{%USERPROFILE}\\.cargo\\bin\\rustup.exe"; Parameters: "target add thumbv7em-none-eabihf"; StatusMsg: "ARM Cortex-M4ターゲットを追加中..."; Flags: waituntilterminated runhidden; Check: FileExists(ExpandConstant('{%USERPROFILE}\\.cargo\\bin\\rustup.exe'))

Filename: "{%USERPROFILE}\\.cargo\\bin\\rustup.exe"; Parameters: "target add riscv32imc-unknown-none-elf"; StatusMsg: "RISC-Vターゲットを追加中..."; Flags: waituntilterminated runhidden; Check: FileExists(ExpandConstant('{%USERPROFILE}\\.cargo\\bin\\rustup.exe'))

; Note: ESP32 (xtensa) requires espup and must be handled separately by the user

[Registry]
; Add application folder to the system PATH if not already present
Root: HKLM; Subkey: "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: NeedsAddPath(ExpandConstant('{app}')); Flags: preservestringtype

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;
