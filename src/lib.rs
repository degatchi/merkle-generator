use ethers::{
    abi::{encode, Address, Token},
    types::U256,
    utils::keccak256,
};
// use ethers::{
//     abi::{AbiEncode, Address, Token},
//     types::U256,
// };
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};

pub mod merkle;

pub fn tree() {
    let leaf_values = ["a", "b", "c", "d", "e", "f"];

    let leaves = vec![
        Sha256::hash(&encode(&vec![
            Token::Address(Address::zero()),
            Token::Uint(U256::from(500)),
        ])),
        Sha256::hash(&encode(&vec![
            Token::Address(Address::zero()),
            Token::Uint(U256::from(250)),
        ])),
        Sha256::hash(&encode(&vec![
            Token::Address(Address::zero()),
            Token::Uint(U256::from(98)),
        ])),
        Sha256::hash(&encode(&vec![
            Token::Address(Address::zero()),
            Token::Uint(U256::from(98)),
        ])),
    ];

    // let leaves: Vec<[u8; 32]> = leaf_values
    //     .iter()
    //     .map(|x| Sha256::hash(x.as_bytes()))
    //     .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    let indices_to_prove = vec![1, 2];

    let leaves_to_prove = leaves.get(1..3).ok_or("can't get leaves to prove").unwrap();

    let merkle_proof = merkle_tree.proof(&indices_to_prove);

    let merkle_root = merkle_tree
        .root()
        .ok_or("couldn't get the merkle root")
        .unwrap();

    // Serialize proof to pass it to the client
    let proof_bytes = merkle_proof.to_bytes();

    // Parse proof back on the client
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    println!("leaf_values {:?}", leaf_values);
    println!("leaves {:?}", leaves);
    println!("merkle_root {:?}", merkle_root);

    assert!(proof.verify(
        merkle_root,
        &indices_to_prove,
        leaves_to_prove,
        leaves.len()
    ));

    let x = hex::encode(merkle_root);
    println!("x: {:?}", x);
}
