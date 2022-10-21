// Run this script with `node merkle_tree.js`.
console.log("Generating merkle tree");

const { MerkleTree } = require("merkletreejs");
const keccak256 = require("keccak256");

let whitelistAddresses = [
  "0X5B38DA6A701C568545DCFCB03FCB875F56BEDDC4",
  "0X5A641E5FB72A2FD9137312E7694D42996D689D99",
//   "0XDCAB482177A592E424D1C8318A464FC922E8DE40",
//   "0X6E21D37E07A6F7E53C7ACE372CEC63D4AE4B6BD0",
//   "0X09BAAB19FC77C19898140DADD30C4685C597620B",
//   "0XCC4C29997177253376528C05D3DF91CF2D69061A",
//   "0xdD870fA1b7C4700F2BD7f44238821C26f7392148",
];

const leafNodes = whitelistAddresses.map(addr => keccak256(addr));
const merkleTree = new MerkleTree(leafNodes, keccak256, { sortPairs: true});

const rootHash = merkleTree.getRoot();
console.log('Whitelist Merkle Tree\n', merkleTree.toString());
console.log("Root Hash: ", rootHash);