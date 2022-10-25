pub fn hash_tokens(inputs: Vec<Token>) -> H256 {
    let x = keccak256(&encode(&inputs));
    H256::from(x)
}