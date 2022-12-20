
# mpgen

## Table of content

- [Installation](#Installation)
- [Usage](#Usage)
- [License](#License)
- [Contributing](#Contributing)
- [TODO](#TODO)

## Installation

## Usage

## License

This program is distributed under the terms of the MIT License.

## Contributing

All contributions are welcome.
Both pull requests and issue reports are always appreciated.
Please make sure that all existing tests pass before submitting a pull request.

## Style

- Macros and constants: MAX_SIZE
- Enums: MyEnum
- Enum Values: MYENUM_VALUE1
- Structs and Typedefs: MyStruct
- Functions: my_function()
- Internal functions: my_internal_function_()

## Syntax Notes

- Input is hex strings
- .. can be used as a wildcard
- any byte token can be multiplied e.g. *2 
- spaces are optional and end a token, but are required after a multiplication
- full example: 'ffAA12..(aa, bb)\*2 cc\*3'
