[Setup]
#define Id "09EB7E1F-3E53-477C-A0C0-9D8B765A59BC"
#define AppName "Some Tank Game"
#define Version "1.2.0"
#define Exe "some-tank-game.exe"


PrivilegesRequired=none
AppId={#Id}
AppName={#AppName}
DefaultGroupName={#AppName}
DefaultDirName={autopf}\{#AppName}
UninstallDisplayIcon={app}\{#Exe}

AppVersion={#Version}
WizardStyle=modern
Compression=lzma2
SolidCompression=yes
OutputDir="dist-installer"
OutputBaseFilename="{#AppName} v{#Version} Setup"
SetupIconFile="assets/icon.ico"

[Files]
Source: "dist/{#Exe}"; DestDir: "{app}"
Source: "dist/config.ini"; DestDir: "{app}"
Source: "dist/assets/*"; DestDir: "{app}/assets"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{userdesktop}\{#AppName}"; Filename: "{app}\{#exe}"