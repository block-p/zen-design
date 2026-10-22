' ZenDesign Desktop Silent Launcher for Windows
' Double-clicking this file starts ZenDesign as a standalone desktop app
' with ZERO terminal/black console windows.

Set WshShell = CreateObject("WScript.Shell")
Set fso = CreateObject("Scripting.FileSystemObject")
strPath = fso.GetParentFolderName(WScript.ScriptFullName)

exePath = strPath & "\server\target\release\zen-design-server.exe"
If Not fso.FileExists(exePath) Then
    exePath = strPath & "\zen-design-server.exe"
End If

If fso.FileExists(exePath) Then
    ' Run completely hidden without command prompt (0 = SW_HIDE)
    WshShell.Run """" & exePath & """", 0, False
    WScript.Sleep 1500
    ' Launch in frameless desktop application window mode
    On Error Resume Next
    WshShell.Run "msedge --app=http://127.0.0.1:7456", 1, False
    If Err.Number <> 0 Then
        WshShell.Run "chrome --app=http://127.0.0.1:7456", 1, False
        If Err.Number <> 0 Then
            WshShell.Run "http://127.0.0.1:7456", 1, False
        End If
    End If
Else
    ' First-time build if release binary is not yet compiled
    WshShell.Run "cmd /c """ & strPath & "\ZenDesign-Windows.bat""", 1, False
End If
