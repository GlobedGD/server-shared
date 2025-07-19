#!/bin/bash

set -e

capnp compile -oc++:schema/generated ./schema/shared.capnp --src-prefix=schema
capnp compile -oc++:schema/generated ./schema/main.capnp --src-prefix=schema
capnp compile -oc++:schema/generated ./schema/game.capnp --src-prefix=schema

# ew
mv ./schema/generated/game.capnp.c++ ./schema/generated/game.capnp.cpp
mv ./schema/generated/main.capnp.c++ ./schema/generated/main.capnp.cpp
mv ./schema/generated/shared.capnp.c++ ./schema/generated/shared.capnp.cpp