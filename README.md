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
### adding custom responses
To add a custom response add the following to the `settings.toml` file:
```
[[response]]
uri = "/uri/to/respond/to"
file = "/path/to/response/file"
```

## Supported file types
Sserver is able to serve any of the following file types:
+ markdown
+ html
+ css
+ jpg, png, ico
+ raw text

## Ports
Run with `-p <port_number>` to run Sserver on specific port.
