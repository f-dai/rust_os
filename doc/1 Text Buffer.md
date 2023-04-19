# Build a Rust Module for Text Buffer

Our current code uses raw pointers to access VGA buffer, which is generally considered unsafe. Now we will make a new module and pack all such unsafe stuff into this separate module.

## VGA Text Buffer Format
Each character on the terminal needs 16 bits in buffer. The first byte is for the character itself, and the second byte determines its color. Detailed format is listed in the following table.

| Bit(s) |       Value      |
|:------:|:----------------:|
| 0-7    | ASCII code point |
| 8-11   | Foreground color |
| 12-14  | Background color |
| 15     | Blink            |

If the 11th bit is set, foreground color will be turned to a "bright" version as follows.

| Number |    Color   | Number + Bright Bit | Bright Color |
|:------:|:----------:|:-------------------:|:------------:|
| 0x0    | Black      | 0x8                 | Dark Gray    |
| 0x1    | Blue       | 0x9                 | Light Blue   |
| 0x2    | Green      | 0xa                 | Light Green  |
| 0x3    | Cyan       | 0xb                 | Light Cyan   |
| 0x4    | Red        | 0xc                 | Light Red    |
| 0x5    | Magenta    | 0xd                 | Pink         |
| 0x6    | Brown      | 0xe                 | Yellow       |
| 0x7    | Light Gray | 0xf                 | White        |

Background color also has the above format (0-7 only). If the 15th bit is set, however, background will be blinking.


