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

## Showcase

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
        "id": 0,
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
            "250000000000000000000",
            "0"
        ]
    },
    {
        "id": 1,
        "inputs": [
            "0x599a9d94b12dd3313211bd1ae9e35a30c0753f5e",
            "0",
            "0"
        ]
    },
    {
        "id": 2,
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
