# Merkle Generator
Bootstrap your merkle tree, in Rust.

# Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)

&nbsp;
# Features
- Merkle Tree creation
- Merkle Proof generation.
- Multi data type compatibility.
- Odd leaf amount compatibility w/o duplication of leaf.

&nbsp;
# Installation
First, make sure that you have [Rust installed](https://www.rust-lang.org/tools/install).
&nbsp;
### Install from source
```
git clone https://github.com/DeGatchi/merkle-generator &&
cd merkle-generator &&
cargo install --path .
```

&nbsp;
# Usage
Now that you have the project installed, you can use the `merkle-generator` command from anywhere in your terminal. By default, `merkle-generator` looks for `inputs.json` in the current directory and outputs to `output.json`.

At any point you can use `merkle-generator --help` to see a list of all commands and options.

```
Usage: merkle-generator [OPTIONS]

Options:
  -i, --input-path <INPUT_PATH>    [default: src/input.json]
  -o, --output-path <OUTPUT_PATH>  [default: src/output.json]
  -h, --help                       Print help information
```

Make sure to have everything as a `String` within the input file. The program converts the `String` into it's corresponding type.

### JSON Input
```json
{
        "types": [  // Make sure to capitalize these types.
            "Address",
            "Uint",
            "Uint"
        ]
    },
    {
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e", // address
            "250000000000000000000", // uint
            "0" // uint
        ]
    },
```


### JSON Output
```json
[
  {
    "inputs": [
      "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e", // address
      "250000000000000000000", // uint
      "0" // uint
    ],
    "proof": [
      "0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801", // bytes32
      "0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84" // bytes32
    ],
    "root": "0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6", // bytes32
    "leaf": "0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a" // bytes32
  },
]
```

&nbsp;
# Contributing
First off, thanks for taking the time to contribute! PRs are welcomed and greatly appreciated <3.