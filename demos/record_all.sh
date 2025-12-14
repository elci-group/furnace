#!/bin/bash
# Run all VHS demos

cd /home/adminx/furnace

echo "🎬 Creating Furnace feature demos with VHS..."
echo ""

# Create demos directory if it doesn't exist
mkdir -p demos

# List of tapes to process
TAPES=(
  "plain"
  "tree"
  "grid"
  "compact"
  "composability"
  "all_styles"
  "help"
)

# Run each tape
for tape in "${TAPES[@]}"; do
  echo "📹 Recording: $tape..."
  vhs demos/$tape.tape
  if [ $? -eq 0 ]; then
    echo "✅ $tape.gif created"
  else
    echo "❌ Failed to create $tape.gif"
  fi
  echo ""
done

echo "🎉 All demos recorded!"
echo "Check the demos/ directory for GIF files"
