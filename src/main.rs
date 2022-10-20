use ethers::{
    abi::{encode, AbiEncode, Token},
    prelude::*,
    utils::keccak256,
};
use merkle_generator::merkle::MerkleTree;
use rs_merkle::{algorithms::Sha256, Hasher};

pub fn hash(inputs: Vec<Token>) -> String {
    let x = Sha256::hash(&encode(&inputs)).encode_hex().replace("0x", "");
    x
}

fn main() {
    let hashes = vec![
        hash(vec![
            Token::Address(Address::zero()),
            Token::Uint(U256::from(500)),
        ]),
        hash(vec![Token::Uint(U256::from(600))]),
        hash(vec![Token::Uint(U256::from(700))]),
        hash(vec![Token::Uint(U256::from(800))]),
    ];

    let merkle_inputs = MerkleTree::new(hashes).unwrap();
    println!("{:#?}", merkle_inputs);

    // let proof = vec![
    //     "e3802336d5db6fe8a80873016a8e4ec07da32f8aff1b18fb2900fc6f2b0f68a2",
    //     "045d4ed6e0fa7cfee80f9f83670300cbb104cac68bb8b1bd3dc89c871e5fcdbe",
    // ];

    // let root = &merkle_inputs[merkle_inputs.len() - 1][0];
    // println!("root {:#?}", root);

    // proof_decode(
    //     &root,
    //     proof,
    //     "104aefa9604fadb80a0277aa3935be473de1ff63f58f08c2f1a0dbe51e8fabf1",
    // );
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
