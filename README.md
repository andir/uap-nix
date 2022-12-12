# Simple Nix-based OS for Cudy X6

Playing around with nixpkgs's cross-compilation support.

## Howto

Connect to serial on the device with 115200n8.

Run `tftpboot 0x4000000; bootm` (the default address collides with the load
addresses in our image).

Profit / debug / cry

## Status

* U-boot works (once it is flashed as an additional u-boot that is
  loaded by the original u-boot).
* Kernel used to work but currently doesn't on 6.0, WIP.
