# cradle-runtime

Runtime for the `cradle` game engine. It's powered by `agb`, using an elegant build system to produce an efficient ROM.

## Building

Builds require rust nightly, and build for the target `thumbv4t-none-eabi`.

## Shipping a .gba file for real hardware

To make a game run on real hardware, you will need to convert the built file into a file suitable for
running on the real thing.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/<your game> -o <your game>.gba
```