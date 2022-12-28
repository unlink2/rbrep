
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

## License

This program is distributed under the terms of the MIT License.

## Contributing

All contributions are welcome.
Both pull requests and issue reports are always appreciated.
Please make sure that all existing tests pass before submitting a pull request.

## Syntax Notes

- Input is hex strings
- ?? can be used as a wildcard
- any byte token can be multiplied e.g. *2 
- spaces are optional and end a token, but are required after a multiplication
- range aa-bb meaning [aa-bb[ 
- full example: 'ffAA12..(aa|bb)\*2 cc\*3'
