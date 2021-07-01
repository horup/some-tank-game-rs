REM set icon of build
tools\rcedit\rcedit-x64.exe "target\release\some-tank-game.exe" --set-icon "assets\icon.ico"

REM make non-installer version
rmdir dist /s /q 2>nul
rmdir dist /s /q 2>nul
mkdir dist
xcopy target\release\*.exe dist\
copy config_release.ini dist\config.ini
xcopy assets dist\assets /E /H /C /I

REM make installer version
tools\iscc\ISCC.exe "installer.iss"

REM set icon of setup.exe
REM rcedit corrupts the setup, commented out
REM tools\rcedit\rcedit-x64.exe "Some Tank Game Setup.exe" --set-icon "assets\icon.ico"