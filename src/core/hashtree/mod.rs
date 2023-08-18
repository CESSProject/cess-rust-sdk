use anyhow::{bail, Result};
use merkle_light::hash::Algorithm;
use merkle_light::merkle::MerkleTree;
use merkle_light::proof::Proof;
use sha2::{Digest, Sha256};
use std::{hash::Hasher, path::PathBuf};

use super::utils::hash::calc_sha256;

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

pub fn build_merkle_root_hash(segment_hash: Vec<String>) -> Result<String> {
    if segment_hash.len() == 1 {
        return Ok(segment_hash[0].clone());
    }

    let mut hashlist = Vec::new();
    for i in (0..segment_hash.len()).step_by(2) {
        if i + 1 >= segment_hash.len() {
            let b = hex::decode(&segment_hash[i])?;
            let hash = calc_sha256(&append_bytes(&b, &b))?;
            hashlist.push(hash);
        } else {
            let b1 = hex::decode(&segment_hash[i])?;
            let b2 = hex::decode(&segment_hash[i + 1])?;
            let hash = calc_sha256(&append_bytes(&b1, &b2))?;
            hashlist.push(hash);
        }
    }

    // Calculate the Merkle root hash directly from the hashlist
    let mut hasher = Sha256::new();
    for hash in &hashlist {
        hasher.update(hash);
    }

    Ok(hex::encode(hasher.finalize()))
}

fn append_bytes(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(b1.len() + b2.len());
    result.extend_from_slice(b1);
    result.extend_from_slice(b2);
    result
}

pub fn build_simple_merkle_root_hash(segment_hash: &str) -> Result<String> {
    let bytes = hex::decode(segment_hash)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}
