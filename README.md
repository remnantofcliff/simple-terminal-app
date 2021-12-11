Disclaimer:
This crate is currently very early in development and probably nearly unusable for its use-case.
---
# About
This crate aims to ease the pain of creating simple console applications that require raw-mode from the terminal. e.g. games, text-editors and such.

Uses the [termion](https://lib.rs/crates/termion) rust-crate.

# Versions

## v. 0.0.3
Now re-exports only parts of termion, some important parts might still be missing. Reading stdin is no longer in a separate thread so that stdin isn't locked forever. Reworked State to ease use. Also removed the helper module from scene at least for now.

## v. 0.0.2
includes termion now in the package, since it's required for using the crate (for now).

## v. 0.0.1
initial release