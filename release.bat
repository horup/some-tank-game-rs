rmdir out /s /q 2>nul
mkdir out
xcopy target\release\*.exe out\
xcopy assets out\assets /E /H /C /I
cd out
tar.exe -a -cf ..\release.zip *.*
cd ..