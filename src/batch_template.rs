/// Single-shot batch (CR+LF, ASCII only)
/// e.g. => 1_myrcbox_margin3_20231101123456.rcbox
pub const FREE_REGION_BAT: &str = "@echo off\r\n\
if \"%ROOTFOLDER%\"==\"\" (\r\n\
    echo [ERROR] ROOTFOLDER not set\r\n\
    exit /b 1\r\n\
)\r\n\
\r\n\
if not exist \"%ROOTFOLDER%\\reconRegion\" mkdir \"%ROOTFOLDER%\\reconRegion\"\r\n\
\r\n\
if \"%RC_PATH%\"==\"\" (\r\n\
    set \"RC_PATH=C:\\Program Files\\Capturing Reality\\RealityCapture\\RealityCapture.exe\"\r\n\
)\r\n\
if \"%PREFIX%\"==\"\" set \"PREFIX=myrcbox\"\r\n\
if \"%MARGIN%\"==\"\" set \"MARGIN=3\"\r\n\
if \"%NUMBER%\"==\"\" set \"NUMBER=1\"\r\n\
\r\n\
echo [INFO] RC_PATH=%RC_PATH%\r\n\
echo [INFO] PREFIX=%PREFIX%\r\n\
echo [INFO] MARGIN=%MARGIN%\r\n\
echo [INFO] NUMBER=%NUMBER%\r\n\
echo [INFO] CMD_PARAM=%CMD_PARAM%\r\n\
\r\n\
set userMargin=%MARGIN%\r\n\
\r\n\
rem Convert margin => marginFactor => revertFactor\r\n\
if \"%userMargin%\"==\"0\" (\r\n\
    set \"marginFactor=1.00\"\r\n\
    set \"revertFactor=1.00\"\r\n\
) else if \"%userMargin%\"==\"1\" (\r\n\
    set \"marginFactor=1.01\"\r\n\
    set \"revertFactor=0.9901\"\r\n\
) else if \"%userMargin%\"==\"2\" (\r\n\
    set \"marginFactor=1.02\"\r\n\
    set \"revertFactor=0.9804\"\r\n\
) else if \"%userMargin%\"==\"3\" (\r\n\
    set \"marginFactor=1.03\"\r\n\
    set \"revertFactor=0.9709\"\r\n\
) else if \"%userMargin%\"==\"4\" (\r\n\
    set \"marginFactor=1.04\"\r\n\
    set \"revertFactor=0.9615\"\r\n\
) else if \"%userMargin%\"==\"5\" (\r\n\
    set \"marginFactor=1.05\"\r\n\
    set \"revertFactor=0.9524\"\r\n\
) else if \"%userMargin%\"==\"6\" (\r\n\
    set \"marginFactor=1.06\"\r\n\
    set \"revertFactor=0.9434\"\r\n\
) else if \"%userMargin%\"==\"7\" (\r\n\
    set \"marginFactor=1.07\"\r\n\
    set \"revertFactor=0.9346\"\r\n\
) else if \"%userMargin%\"==\"8\" (\r\n\
    set \"marginFactor=1.08\"\r\n\
    set \"revertFactor=0.9259\"\r\n\
) else if \"%userMargin%\"==\"9\" (\r\n\
    set \"marginFactor=1.09\"\r\n\
    set \"revertFactor=0.9174\"\r\n\
) else (\r\n\
    set \"marginFactor=1.00\"\r\n\
    set \"revertFactor=1.00\"\r\n\
)\r\n\
\r\n\
if /i \"%CMD_PARAM%\"==\"x+\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -offsetReconstructionRegion 1 0 0\r\n\
    exit /b 0\r\n\
)\r\n\
if /i \"%CMD_PARAM%\"==\"x-\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -offsetReconstructionRegion -1 0 0\r\n\
    exit /b 0\r\n\
)\r\n\
if /i \"%CMD_PARAM%\"==\"y+\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -offsetReconstructionRegion 0 1 0\r\n\
    exit /b 0\r\n\
)\r\n\
if /i \"%CMD_PARAM%\"==\"y-\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -offsetReconstructionRegion 0 -1 0\r\n\
    exit /b 0\r\n\
)\r\n\
if /i \"%CMD_PARAM%\"==\"z+\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -offsetReconstructionRegion 0 0 1\r\n\
    exit /b 0\r\n\
)\r\n\
if /i \"%CMD_PARAM%\"==\"z-\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -offsetReconstructionRegion 0 0 -1\r\n\
    exit /b 0\r\n\
)\r\n\
\r\n\
rem out => export => rename => \"NUMBER_PREFIX_marginN_timestamp.rcbox\"\r\n\
if /i \"%CMD_PARAM%\"==\"out\" (\r\n\
    \"%RC_PATH%\" -delegateTo * -exportReconstructionRegion \"%ROOTFOLDER%\\reconRegion\\temp_noMargin.rcbox\"\r\n\
    if not \"%marginFactor%\"==\"1.00\" (\r\n\
        \"%RC_PATH%\" -delegateTo * -scaleReconstructionRegion %marginFactor% %marginFactor% %marginFactor% center factor\r\n\
    )\r\n\
    \"%RC_PATH%\" -delegateTo * -exportReconstructionRegion \"%ROOTFOLDER%\\reconRegion\\temp_margin.rcbox\"\r\n\
    if not \"%marginFactor%\"==\"1.00\" (\r\n\
        \"%RC_PATH%\" -delegateTo * -scaleReconstructionRegion %revertFactor% %revertFactor% %revertFactor% center factor\r\n\
    )\r\n\
\r\n\
    call :getPreciseTimestamp myExportTime\r\n\
\r\n\
    call :renameOne \"%ROOTFOLDER%\\reconRegion\\temp_noMargin.rcbox\" \"noMargin\" \"%MARGIN%\" \"!myExportTime!\"\r\n\
    call :renameOne \"%ROOTFOLDER%\\reconRegion\\temp_margin.rcbox\"   \"margin\"  \"%MARGIN%\" \"!myExportTime!\"\r\n\
\r\n\
    rem Remove local rcbox if exist\r\n\
    if exist temp_noMargin.rcbox del temp_noMargin.rcbox\r\n\
    if exist temp_margin.rcbox del temp_margin.rcbox\r\n\
\r\n\
    exit /b 0\r\n\
)\r\n\
\r\n\
echo [WARN] unrecognized => %CMD_PARAM%\r\n\
exit /b 0\r\n\
\r\n\
:renameOne\r\n\
setlocal EnableDelayedExpansion\r\n\
\r\n\
set \"rcboxFile=%~1\"\r\n\
set \"tag=%~2\"\r\n\
set \"mval=%~3\"\r\n\
set \"ts=%~4\"\r\n\
\r\n\
if not exist \"!rcboxFile!\" (\r\n\
    endlocal\r\n\
    goto :eof\r\n\
)\r\n\
\r\n\
rem e.g. => 1_myrcbox_margin3_20231101234567.rcbox\r\n\
if \"!mval!\"==\"0\" (\r\n\
    set \"finalName=%NUMBER%_%PREFIX%_!tag!_!ts!.rcbox\"\r\n\
) else (\r\n\
    set \"finalName=%NUMBER%_%PREFIX%_!tag!!mval!_!ts!.rcbox\"\r\n\
)\r\n\
\r\n\
move \"!rcboxFile!\" \"%ROOTFOLDER%\\reconRegion\\!finalName!\" >nul\r\n\
echo [OK] => !finalName!\r\n\
\r\n\
endlocal\r\n\
goto :eof\r\n\
\r\n\
:getPreciseTimestamp\r\n\
setlocal EnableDelayedExpansion\r\n\
\r\n\
set \"rawDate=%date%\"\r\n\
set \"rawTime=%time%\"\r\n\
\r\n\
for /f \"tokens=1-3 delims=/\" %%a in (\"!rawDate!\") do (\r\n\
    set \"yyyy=%%a\"\r\n\
    set \"mm=%%b\"\r\n\
    set \"dd=%%c\"\r\n\
)\r\n\
\r\n\
for /f \"tokens=1-4 delims=:. \" %%i in (\"!rawTime!\") do (\r\n\
    set \"HH=%%i\"\r\n\
    set \"Min=%%j\"\r\n\
    set \"Sec=%%k\"\r\n\
    set \"Ms=%%l\"\r\n\
)\r\n\
\r\n\
set \"theStamp=!yyyy!!mm!!dd!_!HH!!Min!!Sec!!Ms!\"\r\n\
\r\n\
endlocal & set \"%~1=%theStamp%\"\r\n\
goto :eof\r\n";
