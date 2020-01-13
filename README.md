# Sserver

## To Do
+ script support - definitely
+ authorization - possibly
+ databases - possibly

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
A additional settings file can be added to your repository to further specify the routes.

## Configuration
Configuration is done in the `settings.toml` file.
+ `index` - corresponds to what you get when you GET your site's url
+ `skeleton` - a skeleton html file, required for markdown generation
+ `response` - optional, a file describing additional settings; note that this implies that a route must be set or your file won't be served.
### git
+ `url` - url of your git repo
+ `branch` - the branch that should be used
### response format
```toml
[[response]]
uri = "readme"
file = "README.md"
```
This will respond with the `README.md` to `GET` to `readme`.
Note that `readme` != `/readme` since `/` implies root.

## Supported file types
Sserver is able to serve any of the following file types:
+ markdown
+ html
+ css
+ jpg, png, ico
+ raw text
+ any executable file - stdout is returned as html

## Ports
Run with `-p <port_number>` to run Sserver on specific port (8000 is the default).
