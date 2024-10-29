#!/bin/env fish
cargo build && hyprctl dispatch exec "[float] kitty -c ~/.config/kitty/toolbelt.conf $(pwd)/target/debug/toolbelt $argv[1]"
