use std::collections::BTreeMap;

use ethers::{
    abi::{encode, Token},
    prelude::*,
    utils::keccak256,
};
use serde::ser::SerializeMap;
use serde_json::Value;

#[derive(Clone, serde::Deserialize)]
pub struct Data {
    inputs: Vec<Token>,
    proof: Vec<H256>,
    root: H256,
    leaf: H256,
}

impl Data {
    pub fn new(inputs: Vec<Token>, proof: Vec<H256>, root: H256, leaf: H256) -> Self {
        Self {
            inputs,
            proof,
            root,
            leaf,
        }
    }
}

impl serde::Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // We don't know the length of the map at this point, so it's None
        let mut map = serializer.serialize_map(None)?;

        let formatted_inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|token| {
                if let Some(x) = token.clone().into_uint() {
                    x.to_string()
                } else if let Some(x) = token.clone().into_address() {
                    format!("{:?}", x)
                } else {
                    token.to_string()
                }
            })
            .collect();

        map.serialize_entry("inputs", &formatted_inputs)?;
        map.serialize_entry("proof", &self.proof)?;
        map.serialize_entry("root", &self.root)?;
        map.serialize_entry("leaf", &self.leaf)?;

        map.end()
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|token| {
                if let Some(x) = token.clone().into_uint() {
                    x.to_string()
                } else {
                    token.to_string()
                }
            })
            .collect();

        f.debug_struct("Output")
            .field("inputs", &formatted_inputs)
            .field("proof", &self.proof)
            .field("root", &self.root)
            .field("leaf", &self.leaf)
            .finish()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MerkleTree {
    pub token_hash: BTreeMap<H256, Vec<Token>>,
    pub rows: Vec<Vec<H256>>,
    pub product_hashes: BTreeMap<H256, (H256, H256)>,
    pub proofs: BTreeMap<H256, Vec<H256>>,
    pub root: H256,
}

impl MerkleTree {
    pub fn import() -> Self {
        // Get the filenames from the command line.
        let input_path = format!("src/input.json");

        let data = {
            // Load the first file into a string.
            let text = std::fs::read_to_string(&input_path).unwrap();

            // Parse the string into a dynamically-typed JSON structure.
            serde_json::from_str::<Value>(&text).unwrap()
        };

        let elements = data.as_array().unwrap();
        let types = elements[0].get("types").unwrap().as_array().unwrap();

        let mut token_list: Vec<Vec<Token>> = vec![];
        for element in elements.iter().skip(1) {
            let mut tokens: Vec<Token> = vec![];

            let inputs = element.get("inputs").unwrap().as_array().unwrap();
            for (i, input) in inputs.iter().enumerate() {
                match types[i].as_str().unwrap() {
                    "Address" => tokens.push(Token::Address(
                        input.as_str().unwrap().parse::<Address>().unwrap(),
                    )),
                    "Uint" => tokens.push(Token::Uint(
                        U256::from_dec_str(input.as_str().unwrap()).unwrap(),
                    )),
                    _ => println!("Fail conversion"),
                }
            }
            token_list.push(tokens);
        }

        MerkleTree::new(&token_list)
    }

    pub fn new(inputs: &Vec<Vec<Token>>) -> Self {
        let leaf_hashes: BTreeMap<H256, Vec<Token>> = hash_all_tokens(inputs);

        // New hash created by (Hash 1, Hash 2)
        let mut product_hashes: BTreeMap<H256, (H256, H256)> = BTreeMap::new();

        let initial_hashes: Vec<H256> = leaf_hashes.clone().into_keys().collect();

        let mut hashes: Vec<Vec<H256>> = vec![initial_hashes];

        while hashes[hashes.len() - 1].len() != 1 {
            let output = hash(&hashes[hashes.len() - 1].clone(), &mut product_hashes);
            hashes.push(output);
        }

        // println!("Hash products {:#?}", product_hashes);

        let mut tree = Self {
            token_hash: leaf_hashes,
            root: hashes[hashes.len() - 1][0].clone(),
            rows: hashes,
            product_hashes,
            proofs: BTreeMap::new(),
        };

        tree.calculate_proofs();
        tree.record_output();

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

    pub fn record_output(&self) {
        let path: String = format!("src/output.json");
        std::fs::File::create(path.clone()).unwrap();

        let mut data: Vec<Data> = vec![];
        for (leaf, inputs) in self.token_hash.iter() {
            data.push(Data::new(
                inputs.to_vec(),
                self.proofs.get(leaf).unwrap().to_vec(),
                self.root.clone(),
                leaf.clone(),
            ));
        }

        // Save the JSON structure into the output file
        std::fs::write(path, serde_json::to_string_pretty(&data).unwrap()).unwrap();
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

pub fn hash_all_tokens(inputs: &Vec<Vec<Token>>) -> BTreeMap<H256, Vec<Token>> {
    let mut mapping: BTreeMap<H256, Vec<Token>> = BTreeMap::new();

    for tokens in inputs {
        let hash = hash_tokens(tokens);
        mapping.insert(hash, tokens.to_vec());
    }

    mapping
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
    #[ignore]
    fn test_hash_by_leaves() {
        let input_tokens = input_tokens();
        let tree = MerkleTree::new(&input_tokens);
        println!("Merkle Tree: {:#?}", tree)
    }

    #[test]
    // #[ignore]
    fn test_import() {
        let tree = MerkleTree::import();
        println!("Merkle Tree: {:#?}", tree)
    }
}
