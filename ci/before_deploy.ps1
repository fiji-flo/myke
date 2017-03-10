# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path

Set-Location $ENV:Temp

$BINARY = "$SRC_DIR\myke-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).exe"

Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\myke.exe" "$BINARY"


Push-AppveyorArtifact "$BINARY"
