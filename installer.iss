[Setup]
PrivilegesRequired=none
AppId=09EB7E1F-3E53-477C-A0C0-9D8B765A59BC
AppName=Some Tank Game
AppVersion=0.9.0
WizardStyle=modern
DefaultDirName={autopf}\Some Tank Game
DefaultGroupName=Some Tank Game
UninstallDisplayIcon={app}\some-tank-game.exe
Compression=lzma2
SolidCompression=yes
OutputDir="."
OutputBaseFilename="Some Tank Game Setup"

[Files]
Source: "dist/some-tank-game.exe"; DestDir: "{app}"
Source: "dist/config.ini"; DestDir: "{app}"
Source: "dist/assets/*"; DestDir: "{app}/assets"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{userdesktop}\Some Tank Game"; Filename: "{app}\some-tank-game.exe"