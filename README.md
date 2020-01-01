# Sserver

## Dependencies
+ `git`
+ [`rust`](https://www.rust-lang.org/tools/install) - nightly version
+ `make` - not needed, but simplyfies the compilation

## Compilation
### with `make`
```shell
make
```
### without `make`
```shell
cargo build --release
```

## Configuration
Configuration is done in the `settings.toml` file.

## Supported file types
Sserver is able to serve any of the following file types:
+ markdown
+ html
+ css
+ jpg, png, ico
+ raw text

## Usage

