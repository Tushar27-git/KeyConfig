' Theasus Silent Launcher - Launches native Windows application with ZERO cmd window
Set objShell = CreateObject("WScript.Shell")
Set objFSO = CreateObject("Scripting.FileSystemObject")
strDir = objFSO.GetParentFolderName(WScript.ScriptFullName)
strExe = Chr(34) & strDir & "\target\release\theasus.exe" & Chr(34)

' Run window mode 0 (hidden console) so absolutely no terminal window flashes or stays open
objShell.Run strExe, 0, False
Set objShell = Nothing
Set objFSO = Nothing
