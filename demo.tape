Output demo.gif

Require packard

Set Shell "zsh"
Set FontSize 32
Set Width 1200
Set Height 600

Type@250ms "packard" Sleep 500ms
Hide Type " -c 48 -l news" Show Sleep 500ms
Type@200ms " | less" Sleep 500ms Enter
Sleep 3s
PageDown@50ms 1
Sleep 1s
PageDown@50ms 2
Sleep 1s
PageUp@50ms 3
Sleep 3s
