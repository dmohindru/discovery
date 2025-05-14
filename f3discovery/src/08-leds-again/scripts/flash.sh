#!/bin/bash
set -e

# Build and extract ELF path
ELF=$(cargo build --release --target thumbv7em-none-eabihf --message-format=json \
  | jq -r 'select(.executable != null) | .executable' \
  | head -n 1)

echo "Flashing: $ELF"

# Flash using OpenOCD
openocd -s "$HOME/opt/pico/openocd/tcl" \
  -f interface/stlink.cfg \
  -f target/stm32f3x.cfg \
  -c "program $ELF verify reset exit"

echo -e "\n🎉 Flashing successful!"

# Wait for user input before exiting (helps keep terminal/console open)
read -p "🔧 Press ENTER to exit..."
