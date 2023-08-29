# Blckpp

Blckpp is an open-source application that prevents the opening of processes in Linux. With Blckpp, you can block specific processes from running on your system. For example, you can block Chrome from opening by running `blckpp -b chrome`. This way, every time Chrome is opened, Blckpp will close it.

[![Latest Release](https://img.shields.io/github/v/release/codephi/blckpp.svg)](https://github.com/codephi/blckpp/releases/latest)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/codephi/blckpp/build?label=Actions%20Status)


# Install
## CURL
```
curl -sSL https://raw.githubusercontent.com/codephi/blckpp/main/install.sh | bash
```
## WGET
```
wget -qO- https://raw.githubusercontent.com/codephi/blckpp/main/install.sh | bash
```

## Usage

To use Blckpp, run the following command:

### Options

- `-s, --sleep <SLEEP>`: Set the sleep time in seconds between each check for blocked processes. Default is 1 second.
- `-b, --blocked <BLOCKED>`: Block a specific process. For example, `blckpp -b chrome` will block Chrome from opening.
- `-a, --active <ACTIVE>`: Set whether Blckpp is active or not. Possible values are `true` and `false`. Default is `true`.
- `--settings`: Open the settings file in your default text editor.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

### Examples

- Block Chrome from opening: `blckpp -b chrome`
- Block multiple processes: `blckpp -b chrome -b firefox`
- Set the sleep time to 5 seconds: `blckpp -s 5`
- Disable Blckpp: `blckpp -a false`

## License

Blckpp is released under the MIT License. See [LICENSE](LICENSE) for details.
