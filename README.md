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

## Usage
Create a git repository with files to be served in it (html, markdown).
Create a skeleton file inside your repo that has `{}` in place of markdown.
Something along the lines of:
```html
<!DOCTYPE html>
<html>
	<head>
	</head>
	<body>
	{}
	</body>
</html>
```
Edit the `settings.toml` file to make sure Sserver clones your repo with the right branch.
Run Sserver.

To edit your site just edit your repository and your changes will be pulled by Sserver.
As of now Sserver serves all the files in the repository.

## Configuration
Configuration is done in the `settings.toml` file.
+ `index` - corresponds to what you get when you GET your site's url
+ `skeleton` - a skeleton html file, required for markdown generation
### git
+ `url` - url of your git repo
+ `branch` - the branch that should be used
### adding custom responses (not yet)
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
Run with `-p <port_number>` to run Sserver on specific port (8000 is the default).
