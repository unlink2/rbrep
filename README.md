
# mrep

## Table of content

- [Installation](#Installation)
- [Usage](#Usage)
- [License](#License)
- [Contributing](#Contributing)
- [TODO](#TODO)

## Installation

This program requires the latest stable release of rust.
Once rust is set up correclty simply clone the repository.
Then run:

```sh
cargo build # to build or
cargo install  # to install 
```

## Usage

### Syntax

The following options are available as of now:

- any 8 bit hex number (e.g. 1a) will be interpreted as this precise value
- ?? will match any value
- A string (e.g. "a string") will match an exact string
- A range (e.g. 1a-20) will match the range from n..m
- A group will match the first valid item contained in it (e.g. (aabbaa-bb))
- Any expression can be multiplied (e.g. aa*4)

## License

This program is distributed under the terms of the MIT License.

## Contributing

All contributions are welcome.
Both pull requests and issue reports are always appreciated.
Please make sure that all existing tests pass before submitting a pull request.
