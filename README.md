# Merkle Generator

Bootstrap your merkle tree, in Rust.

Please feel free to make PRs <3

## Features

- [x] Multi data type compatibility.
- [x] Odd leaf amount compatibility w/o duplication of leaf.
- [x] Proof generation.
  - [x] `.json` output.
- [x] Merkle Tree creation
  - [x] From `.json` input.

## Quick Start

1. Type in your desired whitelist parameters into `input.json` with the format of, `"types"` followed by `"inputs"`. Example shown in `Showcase`'s `JSON Input`.
2. Run `cargo start` to create the `output.json` with the `input.json`.
3. When using whitelists in your contract, use openzeppelin's `MerkleProof.sol`'s `verify` function. Use the `inputs` to create the `node` (e.g, `bytes32 node = keccak256(abi.encode(inputA, inputB, inputC));`). Then fill out the `verify` parameters with `root`, `node` and `proof`.
4. Integrate the `output.json` into your frontend with connected public key detection to auto-fill the parameters in whatever format you desire.

## Showcase

### How to read the input file

Make sure to have everything as a `String`. The program converts the `String` into it's corresponding type.

E.g, for `"250000000000000000000"`, we use the 2nd element in `"types"` which is `"Uint"`, therefore `"250000000000000000000"` is converted into a `uint256`.

```
{
        "types": [
            "Address",
            "Uint",
            "Uint"
        ]
    },
    {
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
            "250000000000000000000",
            "0"
        ]
    },
```

### JSON Input

```
[
    {
        "types": [
            "Address",
            "Uint",
            "Uint"
        ]
    },
    {
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
            "250000000000000000000",
            "0"
        ]
    },
    {
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
            "0",
            "0"
        ]
    },
    {
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
            "0",
            "125000000000000000000"
        ]
    }
]
```

### Output

```
test tests::test_hash_by_leaves ... ignored
MerkleTree {
    token_hash: {
        0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801: [
            Address(
                0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e,
            ),
            Uint(
                0,
            ),
            Uint(
                125000000000000000000,
            ),
        ],
        0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a: [
            Address(
                0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e,
            ),
            Uint(
                250000000000000000000,
            ),
            Uint(
                0,
            ),
        ],
        0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a: [
            Address(
                0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e,
            ),
            Uint(
                0,
            ),
            Uint(
                0,
            ),
        ],
    },
    product_hashes: {
        0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84: (
            0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a,
            0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a,
        ),
        0x8681a8817cfd31888b4c697a38991e6f3be06accbdb36459e8bf5e92c2eb6600: (
            0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801,
            0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a,
        ),
        0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6: (
            0x8681a8817cfd31888b4c697a38991e6f3be06accbdb36459e8bf5e92c2eb6600,
            0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84,
        ),
    },
    rows: [
        [
            0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801,
            0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a,
            0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a,
        ],
        [
            0x8681a8817cfd31888b4c697a38991e6f3be06accbdb36459e8bf5e92c2eb6600,
            0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84,
        ],
        [
            0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6,
        ],
    ],
    proofs: {
        0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801: [
            0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a,
            0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84,
        ],
        0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a: [
            0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801,
            0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84,
        ],
        0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a: [
            0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a,
            0x8681a8817cfd31888b4c697a38991e6f3be06accbdb36459e8bf5e92c2eb6600,
        ],
    },
    root: 0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6,
}
```

### JSON Output

```
[
  {
    "inputs": [
      "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
      "0",
      "125000000000000000000"
    ],
    "proof": [
      "0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a",
      "0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84"
    ],
    "root": "0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6",
    "leaf": "0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801"
  },
  {
    "inputs": [
      "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
      "250000000000000000000",
      "0"
    ],
    "proof": [
      "0x69d3ca75db69c48c0569d359a5f110f5101ae898fe7a89e9537aa4a487110801",
      "0x5074756108d06d9e89bfa45aa7fcf1ab486e98cffce378a9bc71098e5687cb84"
    ],
    "root": "0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6",
    "leaf": "0x6fcec51a48c67ee2de86adc83fb1d9e65b8b8c8f60548cd839e3c463c9e5a46a"
  },
  {
    "inputs": [
      "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
      "0",
      "0"
    ],
    "proof": [
      "0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a",
      "0x8681a8817cfd31888b4c697a38991e6f3be06accbdb36459e8bf5e92c2eb6600"
    ],
    "root": "0x97c7f98805481c199f21f29a2390071af3f73b91e19797d5a5d6f6c8bed296c6",
    "leaf": "0xd38fcdc03d82a6257cc8de426b5cb3e43a49073781a20be0c80ed1a3372a139a"
  }
]
```
