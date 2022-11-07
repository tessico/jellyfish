// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the Jellyfish library.

// You should have received a copy of the MIT License
// along with the Jellyfish library. If not, see <https://mit-license.org/>.

//! Provides sample instantiations of merkle tree.
//! E.g. Sparse merkle tree with BigUInt index.

use super::{append_only::MerkleTree, DigestAlgorithm, Element, Index, NodeValue};
use crate::rescue::{Permutation, RescueParameter};
use ark_ff::Field;
use ark_std::marker::PhantomData;
use num_bigint::BigUint;
use sha3::{Digest, Sha3_256};
use typenum::U3;

/// Wrapper for rescue hash function
pub struct RescueHash<F: RescueParameter> {
    phantom_f: PhantomData<F>,
}

impl<F: RescueParameter> DigestAlgorithm<F, u64, F> for RescueHash<F> {
    fn digest(data: &[F]) -> F {
        let perm = Permutation::default();
        perm.sponge_no_padding(data, 1).unwrap()[0]
    }

    fn digest_leaf(pos: &u64, elem: &F) -> F {
        let data = [F::from(*pos), *elem, F::zero()];
        let perm = Permutation::default();
        perm.sponge_no_padding(&data, 1).unwrap()[0]
    }
}

/// A standard merkle tree using RATE-3 rescue hash function
pub type RescueMerkleTree<F> = MerkleTree<F, RescueHash<F>, u64, U3, F>;

/// Example instantiation of a SparseMerkleTree indexed by BigUInt
pub type SparseMerkleTree<E, F> = MerkleTree<E, RescueHash<F>, BigUint, U3, F>;

/// Element type for interval merkle tree
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Interval<F: Field>(pub F, pub F);
impl<F: Field> Element for Interval<F> {}

impl<F: RescueParameter> DigestAlgorithm<Interval<F>, u64, F> for RescueHash<F> {
    fn digest(data: &[F]) -> F {
        let perm = Permutation::default();
        perm.sponge_no_padding(data, 1).unwrap()[0]
    }

    fn digest_leaf(pos: &u64, elem: &Interval<F>) -> F {
        let data = [F::from(*pos), elem.0, elem.1];
        let perm = Permutation::default();
        perm.sponge_no_padding(&data, 1).unwrap()[0]
    }
}

/// Interval merkle tree instantiation for interval merkle tree using Rescue
/// hash function.
pub type IntervalMerkleTree<F> = MerkleTree<Interval<F>, RescueHash<F>, u64, U3, F>;

/// Update the array length here
#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Sha3Node([u8; 32]);
impl NodeValue for Sha3Node {}

impl AsRef<[u8]> for Sha3Node {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Wrapper for SHA3_512 hash function
pub struct Sha3Digest();

impl<E: Element, I: Index> DigestAlgorithm<E, I, Sha3Node> for Sha3Digest {
    fn digest(data: &[Sha3Node]) -> Sha3Node {
        let mut hasher = Sha3_256::new();
        for value in data {
            hasher.update(value);
        }
        Sha3Node(hasher.finalize().into())
    }

    fn digest_leaf(_pos: &I, _elem: &E) -> Sha3Node {
        // Serialize and hash
        todo!()
    }
}

/// Merkle tree using SHA3 hash
pub type SHA3MerkleTree<E> = MerkleTree<E, Sha3Digest, u64, U3, Sha3Node>;