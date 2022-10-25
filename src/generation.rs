use std::collections::BTreeMap;

use ethers::{
    abi::{encode, Token},
    prelude::*,
    utils::keccak256,
};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: H256,
    pub initial_tokens: Vec<Vec<Token>>,
    pub rows: Vec<Vec<H256>>,
    pub product_hashes: BTreeMap<H256, (H256, H256)>,
    pub proofs: BTreeMap<H256, Vec<H256>>,
}

impl MerkleTree {
    pub fn new(inputs: &Vec<Vec<Token>>) -> Self {
        let initial_hashes = hash_all_tokens(inputs);

        // New hash created by (Hash 1, Hash 2)
        let mut product_hashes: BTreeMap<H256, (H256, H256)> = BTreeMap::new();

        let mut hashes: Vec<Vec<H256>> = vec![initial_hashes];
        while hashes[hashes.len() - 1].len() != 1 {
            let output = hash(&hashes[hashes.len() - 1].clone(), &mut product_hashes);
            hashes.push(output);
        }

        println!("Hash products {:#?}", product_hashes);

        let mut tree = Self {
            root: hashes[hashes.len() - 1][0].clone(),
            initial_tokens: inputs.to_vec(),
            rows: hashes,
            product_hashes,
            proofs: BTreeMap::new(),
        };

        tree.calculate_proofs();

        tree
    }

    pub fn calculate_proofs(&mut self) {
        // Initialize each leaf.
        for hash in self.rows[0].iter() {
            self.proofs.insert(*hash, vec![]);
        }

        for (initial, proof) in self.proofs.iter_mut() {
            let mut to_match: H256 = *initial;
            'outer: loop {
                println!("Loop {}", initial);
                // Find matching h1 or h2 until no remain.
                'inner: for (i, (product, (h1, h2))) in self.product_hashes.iter().enumerate() {
                    // If `to_match` is either h1 or h2, add the opposite.
                    // Otherwise, it wasn't found, move onto next product iter.
                    if h1 == &to_match {
                        proof.push(*h2);
                        to_match = *product;
                        break 'inner;
                    } else if h2 == &to_match {
                        proof.push(*h1);
                        to_match = *product;
                        break 'inner;
                    }

                    // If didn't find any match from iter, finish.
                    if i == self.product_hashes.len() - 1 {
                        break 'outer;
                    }
                }
            }
        }
    }
}

pub fn hash(inputs: &Vec<H256>, products: &mut BTreeMap<H256, (H256, H256)>) -> Vec<H256> {
    let mut inputs = inputs.clone();

    // If odd amount of nodes, clone the last one on the end to make even.
    if inputs.len() % 2 != 0 {
        inputs.push(inputs[inputs.len() - 1].clone());
    }

    // Keep track of all the nodes.
    let mut i: usize = 0;
    let mut hashes: Vec<H256> = vec![];

    // Go through each hash pair and create new nodes.
    while hashes.len() != inputs.len() / 2 {
        
        #[allow(unused_assignments)]
        let mut data: Vec<u8> = vec![];

        // Mimicking MerkleProof.sols' _hashPair()` function.
        // Checks which hash is longer, then hashes the smallest first.
        if inputs[i] < inputs[i + 1] {
            data = inputs[i].as_bytes().to_vec();
            data.append(&mut inputs[i + 1].as_bytes().to_vec());
        } else {
            data = inputs[i + 1].as_bytes().to_vec();
            data.append(&mut inputs[i].as_bytes().to_vec());
        }

        let hash = H256::from(keccak256(data));

        products.insert(hash, (inputs[i], inputs[i + 1]));

        hashes.push(hash);

        i += 2;
    }

    hashes
}

pub fn hash_tokens(input: &Vec<Token>) -> H256 {
    H256::from(keccak256(&encode(&input)))
}

pub fn hash_all_tokens(inputs: &Vec<Vec<Token>>) -> Vec<H256> {
    let mut hashes = vec![];
    for tokens in inputs {
        hashes.push(hash_tokens(tokens));
    }
    hashes
}

// impl std::fmt::Debug for MerkleTree {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Node").field("hash", &self.hash).finish()
//     }
// }

// ---------------------------------------------------------------------------------------------
//  Tests
//  run: cargo test -- --nocapture
// ---------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    pub fn input_tokens() -> Vec<Vec<Token>> {
        let deployer = Address::from(
            "0x599A9d94b12dD3313211BD1AE9E35a30c0753f5E"
                .parse::<Address>()
                .unwrap(),
        );

        vec![
            vec![
                Token::Address(deployer),
                Token::Uint(U256::from(250) * U256::exp10(18)),
                Token::Uint(U256::from(0)),
            ],
            vec![
                Token::Address(deployer),
                Token::Uint(U256::from(0)),
                Token::Uint(U256::from(125) * U256::exp10(18)),
            ],
            vec![
                Token::Address(deployer),
                Token::Uint(U256::from(0)),
                Token::Uint(U256::from(0)),
            ],
        ]
    }

    #[test]
    // #[ignore]
    fn test_hash_by_leaves() {
        let input_tokens = input_tokens();
        let tree = MerkleTree::new(&input_tokens);
        println!("Merkle Tree: {:#?}", tree)
    }
}
