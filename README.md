# Rust HCI

This library provides a rust parser for the Bluetooth 
[Host Controller Interface protocol](https://en.wikipedia.org/wiki/List_of_Bluetooth_protocols#HCI)
(HCI), which is used for communication between a controller (bluetooth chip) and
host (a phone or PC, for example).

This library is likely to only be useful to you if you are implementing a bluetooth
stack yourself. If you are looking for a high-level library to build bluetooth
applications in Rust, take a look at [rumble](https://crates.io/crates/rumble).   

Currently this library is alpha quality. There are currently very few tests,
and large parts of the protocol are not yet implemented.
