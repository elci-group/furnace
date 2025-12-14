# Demo: All Styles Showcase
Output demos/all_styles.gif

Set Shell bash
Set FontSize 12
Set Width 1400
Set Height 1000
Set Padding 20
Set Theme "Dracula"

Type "# Furnace: 10 Output Styles Showcase"
Sleep 1s
Enter
Enter

Type "# 1. Plain (default)"
Enter
Type "cargo run -- . --plain | head -15"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# 2. Tree"
Enter
Type "cargo run -- . --tree | head -15"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# 3. Compact"
Enter
Type "cargo run -- . --compact | head -10"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# 4. Grid"
Enter
Type "cargo run -- . --grid"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# 5. Minimal"
Enter
Type "cargo run -- . --minimal | head -10"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# 6. Verbose"
Enter
Type "cargo run -- . --verbose | head -15"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# 7. Badges"
Enter
Type "cargo run -- . --badges | head -15"
Enter
Sleep 3s

Type "clear"
Enter
Sleep 300ms

Type "# All 10 styles available!"
Sleep 2s
