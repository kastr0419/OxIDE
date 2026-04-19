Set WshShell = CreateObject("WScript.Shell")
Dim scriptDir
scriptDir = Left(WScript.ScriptFullName, InStrRev(WScript.ScriptFullName, "\"))
WshShell.Run Chr(34) & scriptDir & "target\release\oxide.exe" & Chr(34), 0, False
