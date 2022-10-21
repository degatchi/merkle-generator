use ethers::{
    abi::{encode, AbiEncode, Token},
    prelude::*,
    utils::keccak256,
};
use merkle_generator::merkle::MerkleTree;

pub fn hash(inputs: Vec<Token>) -> String {
    let x = keccak256(&encode(&inputs)).encode_hex();
    x
}

fn main() {
    let hashes = vec![
        hash(vec![
            Token::Address(Address::from(
                "0xdcD49C36E69bF85FA9c5a25dEA9455602C0B289e"
                    .parse::<Address>()
                    .unwrap(),
            )),
            Token::Uint(U256::from(500)),
        ]),
        hash(vec![Token::Uint(U256::from(600))]),
        hash(vec![Token::Uint(U256::from(700))]),
        hash(vec![Token::Uint(U256::from(800))]),
    ];

    let merkle_inputs = MerkleTree::new(hashes).unwrap();
    println!("{:#?}", merkle_inputs);
}

pub fn proof_decode(root: &str, proof: Vec<&str>, leaf: &str) -> bool {
    let mut last_hash: String = leaf.to_string();
    for i in proof.iter() {
        println!("Hash: {:?}", last_hash);
        last_hash = keccak256(encode(&vec![
            Token::String(last_hash.to_string()),
            Token::String(i.to_string()),
        ]))
        .encode_hex()
        .replace("0x", "")
    }

    if root == last_hash {
        println!("[PASS] root: {:?}", last_hash);
        return true;
    } else {
        println!("[FAIL] root: {:?}", last_hash);
        return false;
    }
}

// ---------------------------------------------------------------------------------------------
//
//  Tests
//  run: cargo test -- --nocapture
//
// ---------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_tree() {
        let addr = Address::from(
            "0xdcD49C36E69bF85FA9c5a25dEA9455602C0B289e"
                .parse::<Address>()
                .unwrap(),
        );
        let y = keccak256(addr.as_bytes());
        println!("addr {:?}", y.encode_hex());

        let amount = encode(&vec![Token::Address(addr), Token::Uint(U256::from(100))]);
        println!("amount: {:?}", keccak256(amount).encode_hex());

        let hashes = vec![
            hash(vec![Token::Address(addr), Token::Uint(U256::from(500))]),
            hash(vec![
                Token::Address(Address::zero()),
                Token::Uint(U256::from(600)),
            ]),
            hash(vec![
                Token::Address(Address::zero()),
                Token::Uint(U256::from(700)),
            ]),
            hash(vec![
                Token::Address(Address::zero()),
                Token::Uint(U256::from(800)),
            ]),
        ];

        let merkle_inputs = MerkleTree::new(hashes).unwrap();
        println!("\n{:#?}", merkle_inputs);
    }
}
