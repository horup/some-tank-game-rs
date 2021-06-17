rmdir dist /s /q 2>nul
rmdir temp /s /q 2>nul
mkdir temp
xcopy target\release\*.exe temp\
xcopy assets temp\assets /E /H /C /I

mkdir dist
cd temp
tar.exe -a -cf ..\dist\release.zip *.*
cd ..