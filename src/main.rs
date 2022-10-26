use merkle_generator::MerkleTree;

fn main() {
    println!("Generating the Merkle Tree.");
    MerkleTree::import();
    println!("DONE: The output is found at src/output.json");
}
