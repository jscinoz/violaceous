#!/bin/sh
cargo run -- /usr/include/libpurple/purple.h -builtins $(pkg-config --cflags glib-2.0) -I /usr/lib/clang/3.7.1/include -I /usr/include/libpurple
