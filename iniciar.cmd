@echo off
TITLE Anestesia Launcher
CLS
fsutil dirty query %systemdrive% >nul
if %errorlevel% NEQ 0 (
    echo [!] Solicitando permisos de Administrador...
    powershell -Command "Start-Process cmd -ArgumentList '/k,call,\"%~f0\"' -Verb RunAs"
    exit
)
cd /d "%~dp0"
echo.
echo [ Anestesia: Windows Sovereignty Tool ]
echo [ Directorio: "%~dp0" ]
echo ---------------------------------------
echo.
IF EXIST "anestesia.exe" (
    echo [OK] Ejecutando comando 'status'...
    echo.
    anestesia.exe status
    echo.
    echo ---------------------------------------
    echo [INFO] Para inyectar la vacuna, escribe abajo: anestesia.exe lock
) ELSE (
    echo [ERROR CRITICO]
    echo No encuentro "anestesia.exe".
    echo.
)
pause
