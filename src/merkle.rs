use ethers::{
    abi::{encode, AbiEncode, Token},
    prelude::*,
    utils::keccak256,
};

pub struct MerkleTree {
    pub root: String,
}

impl MerkleTree {
    pub fn new(inputs: Vec<H256>) -> Option<Vec<Vec<H256>>> {
        if inputs.len() % 2 != 0 {
            return None;
        }

        let mut hashes: Vec<Vec<H256>> = vec![inputs];
        while hashes[hashes.len() - 1].len() != 1 {
            let output = hash(&mut hashes[hashes.len() - 1].clone());
            hashes.push(output);
        }

        return Some(hashes);
    }
}

pub fn hash(inputs: &mut Vec<H256>) -> Vec<H256> {
    if inputs.len() % 2 != 0 {
        inputs.push(inputs[inputs.len() - 1].clone());
    }

    let mut i: usize = 0;
    let mut hashes: Vec<H256> = vec![];

    while hashes.len() != inputs.len() / 2 {
        let mut data: Vec<u8> = vec![];
        if inputs[i] < inputs[i + 1] {
            println!("a < b");
            data = inputs[i].as_bytes().to_vec();
            data.append(&mut inputs[i + 1].as_bytes().to_vec());
        } else {
            println!("a >= b");
            data = inputs[i + 1].as_bytes().to_vec();
            data.append(&mut inputs[i].as_bytes().to_vec());
        }

        hashes.push(H256::from(keccak256(data)));
        i += 2;
    }

    hashes
}
