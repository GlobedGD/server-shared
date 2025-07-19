# Shared server code

This repository contains multiple elements shared by Globed central server, game server and client.

## Protocol definitions

`schema` subfolder contains the Cap 'n Proto schemas that are used by Globed.

* `schema/main.capnp` - The protocol used for client <-> central server communication
* `schema/game.capnp` - The protocol used for client <-> game server communication
* `schema/srvc.capnp` - The protocol used for central server <-> game server communication

If appropriate features are enabled (`main`, `game` and/or `srvc`), these schemas are codegenned at build time into Rust code.

Rust codegenned files are not included, but C++ are (because screw CMake), in `schema/generated`