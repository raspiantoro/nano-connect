# Nano Connect

This repository contains a collection of source code examples for the [Arduino Nano RP2040 Connect](https://docs.arduino.cc/hardware/nano-rp2040-connect/), written in Rust. The examples demonstrate how to leverage Rust's capabilities to interact with the various hardware features of the Arduino Nano RP2040 Connect, such as GPIO, LED control, and USB communication, providing a foundation for building more advanced projects.

## Getting Started

If you want to try out on one of our examples, You need to have the latest stable version of Rust installed, along with the appropriate target support. Follow this [link](https://github.com/rp-rs/rp-hal?tab=readme-ov-file#getting-started) to complete the setup. 

Since all of the examples in this repository use the Arduino Nano Connect BSP, you might also want to check these [docs](https://github.com/rp-rs/rp-hal-boards/tree/main/boards/arduino_nano_connect).

You can run the examples with the following command

```bash
cargo run --release -p blink
```

> [!warning]
>
> After uploading these examples to your board, you will no longer be able to upload through USB because rp-hal and rp-hal-boards do not include the default Arduino Nano bootloader.
> To restore your board to the original bootloader, you need to connect the GND and REC pins with a jumper wire.
> Follow this [link](https://support.arduino.cc/hc/en-us/articles/4404168794514-Nano-RP2040-Connect-not-being-detected-on-USB-port) and [this](https://docs.arduino.cc/tutorials/nano-rp2040-connect/rp2040-01-technical-reference/?_gl=1*15xbpg7*_gcl_au*MTU1MDg0MzUzNy4xNzI0NzY4NDIz*FPAU*MTU1MDg0MzUzNy4xNzI0NzY4NDIz*_ga*OTQ1NjI5MDg3LjE3MjQ3Njg0MTg.*_ga_NEXN8H46L5*MTcyNDkyNjMyOC4xNi4xLjE3MjQ5MjYzMzcuMC4wLjEyOTkyOTA1ODc.*_fplc*JTJGYVVLMmxHUTR1djVuTDZ3dThWemFVUWVxWSUyQjIyR0FMb0VQNmlxTzJxdHRmN2hMdE9kQWdtbFRZc09iVkhVbzBoZ25SY1BPdWk3dHBVakNnOFkxRmp6aWpNanglMkIyZnY4WTNNNU1zJTJCSHVwZ0RrVjA4UkZ0SDB4NG9OZXByb1ElM0QlM0Q.#board-not-detected) if encounter any problems.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
