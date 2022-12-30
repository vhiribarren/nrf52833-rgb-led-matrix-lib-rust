# LED Matrix usage with BBC Microbit

## Deployment

To deploy, once all tools from the Rust discovery book are installed:

    $ cargo embed

## Resources

- [Adafruit] - Wiring and working guide, aims Arduino
- [hzeller GitHub] - Wiring and working guide, also explain some issues if using a 3.3V logic circuits like with Cortex-M.
- [Big Mess o' Wires] - Basic working example, which helped understand the correct OE/Line/Latch sequence and confirmed 
  some issues I had when not modifying the line selection.

[Adafruit]: https://learn.adafruit.com/32x16-32x32-rgb-led-matrix/overview
[Big Mess o' Wires]: https://www.bigmessowires.com/2018/05/24/64-x-32-led-matrix-programming/
[hzeller GitHub]: https://github.com/hzeller/rpi-rgb-led-matrix

## License

MIT License

Copyright (c) 2022 Vincent Hiribarren

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
