### Commands

openocd -s /home/dhruv/opt/pico/openocd/tcl -f interface/stlink.cfg -f target/stm32f3x.cfg

Command to extract executable name from cargo build

```shell
cargo build --target thumbv7em-none-eabihf --message-format=json 2>/dev/null   | jq -r 'select(.executable != null) | .executable'   | head -n 1
```

### Steps

1. Start ocd via script file -- done
2. Launch gdb-multiarch to load the program (run config)
