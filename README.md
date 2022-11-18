# Conway's Game of Life in Rust

## Rationale
Just a bit of retro fun as a rust learning exercise.

![Screen shot](https://www.martyndavis.com/wp-content/uploads/2022/11/life.png "")

## Pre-requisites
### Linux (including WSL)

Ensure you have `libcsfml-dev` and `libsfml-dev` installed.

If you are running this from WSL, and you see errors like the following,

```
Failed to create an OpenGL context for this window
X Error of failed request:  BadValue (integer parameter out of range for operation)
```

then try doing `export LIBGL_ALWAYS_INDIRECT=0` in your bash shell first.
### Mac

`brew install sfml` and `brew install csfml` (install homebrew if you haven't already)

then `export SFML_INCLUDE_DIR=/System/Volumes/Data/opt/homebrew/include` and `export SFML_LIBS_DIR=/System/Volumes/Data/opt/homebrew/lib`


## Building
Just the usual `cargo build`.

## Running
Just the usual `cargo run`.

## Running unit tests
Just the usual `cargo test`
