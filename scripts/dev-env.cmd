@echo off
setlocal
cd /d "%~dp0.."
if exist ".tools\node24\bin\node.exe" set "PATH=%CD%\.tools\node24\bin;%PATH%"
node -e "process.exit(process.versions.node.split('.')[0] === '24' ? 0 : 1)" || (echo Node 24.x required & exit /b 1)
call %*
exit /b %ERRORLEVEL%
