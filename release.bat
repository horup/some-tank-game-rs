rmdir out /s /q 2>nul
mkdir out
xcopy target\release\*.exe out\
xcopy assets out\assets /E /H /C /I