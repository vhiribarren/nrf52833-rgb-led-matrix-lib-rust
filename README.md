# RGB LED Matrix usage with BBC Microbit

![used elements](docs/bonne_annee_material.jpg)

The needed elements are:

- a 64x32 [Adafruit matrix panel]
- a BBC micro:bit v2
- an edge adapter for the micro:bit to plug some Dupont cables
- a [Adafruit RGB Matrix bonnet] to convert the 3.3V micro:bit GPIO outputs to 5V 
- various Dupont cables, that I made to select the size and the required male/female connectors
- there is also a 5V DC power supply which can provide up to 10 A

[Adafruit matrix panel]: https://learn.adafruit.com/32x16-32x32-rgb-led-matrix/overview
[Adafruit RGB Matrix bonnet]: https://learn.adafruit.com/adafruit-rgb-matrix-bonnet-for-raspberry-pi/overview


## Deployment

To deploy, you must set up the tools described in the Rust [Discovery Book]. Notably:
- Rust (obviously)
- Cargo Embed
- Arm GCC toolchain
- The Rust/LLVM target `thumbv7em-none-eabihf` for micro:bit v2

**This project is already configured to be specifically deployed on the micro:bit v2.
The memory map is notably prepared for the nrf52833 chip.**

Once they are installed:

    $ cargo embed --release

Unless you need to debug, **you must use the release version**. The matrix needs
frequent refreshes, which will be impeded by the unoptimized code version.

Some examples are also presents in the `examples/` directory. To use them:

    $ cargo embed --release --example hello_world_blink

[Discovery Book]: https://docs.rust-embedded.org/discovery/


## Resources

What mainly helped me is:

- [Adafruit matrix panel] - Wiring and working guide, aims Arduino
- [hzeller GitHub] - Wiring and working guide, also explain some issues if using a 3.3V logic circuits like with Cortex-M.
- [Big Mess o' Wires] - Basic working example, which helped understand the correct OE/Line/Latch sequence and confirmed 
  some issues I had when not modifying the line selection.
- [Adafruit matrix panel code] - Give some clues on using binary code modulation for LED colors

Some additional related resources:
- [Binary code modulation explanation]
- [Rust port of code for Rasperry Pi] (this one requires an operating system)

[Adafruit matrix panel code]: https://github.com/adafruit/RGB-matrix-Panel
[Big Mess o' Wires]: https://www.bigmessowires.com/2018/05/24/64-x-32-led-matrix-programming/
[hzeller GitHub]: https://github.com/hzeller/rpi-rgb-led-matrix
[Binary code modulation explanation]: http://www.batsocks.co.uk/readme/art_bcm_1.htm
[Rust port of code for Rasperry Pi]: https://github.com/EmbersArc/rpi_led_panel


## Development

### Debug and logging

To debug and display some metrics, the `logging` feature can be enabled.

In order to display the logs using `cargo embed`, you can create a
`Embed.local.toml` local file, and override some elements from `Embed.toml` to
enable RTT.

    [default.rtt]
    enabled = true

Then you can recompile and inject the code by activating the `logging` feature:

    $ cargo embed --release --features logging

### Pre-commit hooks

Some git [pre-commit] hooks are available. You can install them using:

    $ pre-commit install

[pre-commit]: https://pre-commit.com/


## Various notes

- For now, code not optimized; notably, GPIO are manipulated one at a time
  through the HAL instead of manipulating them at once through a direct register
  access, and I do not really care on precise timings
- Binary Code Modulation is implemented to have more than 8 colors. For now, I
  only manage to use 2 BCM bit plans with the Microbit, but it should allow
  about 64 colors? Anyay, the screen is not refreshed quickly enough to allow
  more colors. No gamma correction.
- I stick to heapless development, which make it a bit hard to design something
  that can adapt to various LED matrix sizes
- Currrently, globally hard-coded for a 64x32 RGB LED matrix, I have some issues
  in designing something that can be adapted to other kind of LED Matrix
  (probably procedural macros are the way, or using a heap and trait objects?)
- I have issues in writting some unit tests, with the message `can't find crate
  for 'test'`. Possibly related to the use of `no_std` target.
- Issues in using `defmt` for logging, I have tildes as output. For now, directly
  using rtt with some macro when features `logging` is enabled

## License

MIT License

Copyright (c) 2022, 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
