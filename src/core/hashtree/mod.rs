use merkle_light::hash::Algorithm;
use merkle_light::merkle::MerkleTree;
use merkle_light::proof::Proof;
use sha2::{Digest, Sha256};
use std::hash::Hasher;

pub struct Sha256Algo(Sha256);

impl Sha256Algo {
    pub fn new() -> Self {
        Sha256Algo(Sha256::new())
    }
}

impl Default for Sha256Algo {
    fn default() -> Self {
        Sha256Algo::new()
    }
}

impl Hasher for Sha256Algo {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.update(msg)
    }

    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8; 64]> for Sha256Algo {
    fn hash(&mut self) -> [u8; 64] {
        let mut h = [0u8; 64];
        h.copy_from_slice(self.0.clone().finalize().as_slice());
        h
    }
}

pub fn build_tree(items: Vec<[u8; 64]>) -> MerkleTree<[u8; 64], Sha256Algo> {
    MerkleTree::from_iter(items)
}

pub fn get_root(tree: &MerkleTree<[u8; 64], Sha256Algo>) -> [u8; 64] {
    tree.root()
}

pub fn gen_proof(tree: &MerkleTree<[u8; 64], Sha256Algo>, index: usize) -> Proof<[u8; 64]> {
    tree.gen_proof(index)
}

pub fn validate_proof(proof: &Proof<[u8; 64]>, leaf: &[u8; 64], root: &[u8; 64]) -> bool {
    proof.validate::<Sha256Algo>() && proof.item().as_ref() == leaf && proof.root().as_ref() == root
}