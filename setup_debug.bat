mklink /D target\debug\deps\assets ..\..\..\assets
xcopy %HOMEPATH%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\std**.dll target\debug\deps\