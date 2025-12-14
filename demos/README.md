# Furnace Feature Demos

This directory contains VHS-recorded demonstrations of all Furnace features.

## 🎬 Available Demos

- **plain.gif** - Plain output (default, grep-able)
- **tree.gif** - Tree view with colors and emojis
- **grid.gif** - Table/grid layout
- **compact.gif** - Compact, dense output
- **composability.gif** - Combining presets with modifiers
- **all_styles.gif** - Showcase of all 10 styles
- **help.gif** - Help text and feature overview

## 📝 VHS Tape Files

Each `.tape` file is a VHS script that generates the corresponding `.gif`:

```bash
# Record a single demo
vhs plain.tape

# Record all demos
./record_all.sh
```

## 🚀 Quick View

### Plain Output
Simple, grep-able text - perfect for CI/CD pipelines.

### Tree Output
Hierarchical view with Unicode symbols and emoji indicators:
- 📄 Files
- 🔧 Functions
- 🏗️ Structs
- 🧩 Enums

### Grid Output
ASCII table for quick statistics comparison.

### Composability
Mix presets with modifiers for custom output:
- `--tree --detail verbose`
- `--layout grid --color standard`

## 🎯 Target Codebase

**All demos use Furnace analyzing itself!**

This demonstrates real-world utility while showcasing features.

## 🛠️ Recording Your Own

1. Edit the `.tape` files to customize commands
2. Run `vhs <tape-name>.tape`
3. Share the generated `.gif`!

### VHS Resources

- [VHS Documentation](https://github.com/charmbracelet/vhs)
- [VHS Examples](https://github.com/charmbracelet/vhs/tree/main/examples)

## 📊 Demo Statistics

| Demo | Size | Duration | Features Shown |
|------|------|----------|----------------|
| plain | ~300KB | ~7s | Default output, simplicity |
| tree | ~320KB | ~7s | Colors, emojis, hierarchy |
| grid | ~200KB | ~5s | Table layout, stats |
| compact | ~150KB | ~4s | Dense format |
| composability | ~500KB | ~12s | Modifiers, combinations |

---

**Pro Tip**: Use these GIFs in your README, documentation, or presentations to showcase Furnace's capabilities!
