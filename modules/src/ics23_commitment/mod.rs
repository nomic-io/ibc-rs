use serde_derive::{Deserialize, Serialize};

use crate::path::Path;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentRoot;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPath;

impl CommitmentPath {
    pub fn from_path<P>(_p: P) -> Self
    where
        P: Path,
    {
        todo!()
    }
}

pub type CommitmentProof = tendermint::merkle::proof::Proof;
/*
impl CommitmentProof {
    pub fn from_bytes(_bytes: &[u8]) -> Self {
        todo!()
    }
}
*/

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPrefix;

// TODO: impl CommitPrefix
