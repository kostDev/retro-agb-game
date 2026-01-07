#!/bin/bash
cargo build --release
agb-gbafix target/thumbv4t-none-eabi/release/hero -o hero.gba
