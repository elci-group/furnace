# Demo: Composability
Output demos/composability.gif

Set Shell bash
Set FontSize 14
Set Width 1400
Set Height 900
Set Padding 20
Set Theme "Dracula"

Type "# Furnace Demo: Composability"
Sleep 1s
Enter

Type "# Combine presets with modifiers"
Sleep 1s
Enter
Enter

Type "# Tree layout with verbose detail"
Sleep 1s
Enter
Type "cargo run -- . --tree --detail verbose"
Sleep 500ms
Enter

Sleep 6s

Type "clear"
Enter
Sleep 500ms

Type "# Grid layout with standard color"
Sleep 1s
Enter
Type "cargo run -- . --layout grid --color standard"
Sleep 500ms
Enter

Sleep 4s
