use ethers::{
    abi::{encode, AbiEncode, Token},
    utils::keccak256,
};

pub struct MerkleTree {
    pub root: String,
}

impl MerkleTree {
    pub fn new(inputs: Vec<String>) -> Option<Vec<Vec<String>>> {
        if inputs.len() % 2 != 0 {
            return None;
        }

        let mut hashes: Vec<Vec<String>> = vec![inputs];
        while hashes[hashes.len() - 1].len() != 1 {
            let output = hash(&mut hashes[hashes.len() - 1].clone());
            hashes.push(output);
        }

        return Some(hashes);
    }
}

pub fn hash(inputs: &mut Vec<String>) -> Vec<String> {
    if inputs.len() % 2 != 0 {
        inputs.push(inputs[inputs.len() - 1].clone());
    }

    let mut i: usize = 0;
    let mut hashes: Vec<String> = vec![];
    while hashes.len() != inputs.len() / 2 {
        let data = keccak256(encode(&vec![
            Token::String(inputs[i].clone()),
            Token::String(inputs[i + 1].clone()),
        ]));

        let hex = data.encode_hex();
        // let hex = hex.replace("0x", "");
        hashes.push(hex);
        i += 2;
    }

    hashes
}
