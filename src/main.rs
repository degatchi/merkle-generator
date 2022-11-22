use clap::Parser;
use merkle_generator::MerkleTree;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t  = String::from("src/input.json"))]
    input_path: String,

    #[arg(short, long, default_value_t  = String::from("src/output.json"))]
    output_path: String,
}

fn main() {
    let args = Args::parse();

    println!("Generating the Merkle Tree...");

    MerkleTree::import(&args.input_path, &args.output_path);

    println!("DONE: The output is found at {}", args.output_path);
}
