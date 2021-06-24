rmdir dist /s /q 2>nul
rmdir dist /s /q 2>nul
mkdir dist
xcopy target\release\*.exe dist\
copy config_default.ini dist\config.ini
xcopy assets dist\assets /E /H /C /I