
# rbrep

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

![Gif showing rbrep in action](https://raw.githubusercontent.com/unlink2/misc-resources/main/rbrep-usage.gif)

### Syntax

The following options are available as of now:

- any 8 bit hex number (e.g. 1a) will be interpreted as this precise value
- ?? will match any value
- A string (e.g. "a string") will match an exact string
- A range (e.g. 1a-20) will match the range from n..m
- A group will match the first valid item contained in it (e.g. (aabbaa-bb))
- A group and also be matched using a logical and (e.g. &(aabb))
- A bitwise and (e.g. &A1)
- A not operator (e.g. !31)
- Any expression can be multiplied (e.g. aa*4)
- An expression can be made optional by multiplying with 0 (e.g. aa*0;)
- An expression can occur 1 to n times by adding a + (e.g. aa*1+;)
- Optional and 1 to n can be combined (e.g. aa*0+;)

## License

This program is distributed under the terms of the MIT License.

## Contributing

