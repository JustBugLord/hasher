mod sha2;
mod md;

use std::str::FromStr;
use crate::errors::HashError;

/// The `HashAlgorithm` enumeration contains a list of all implemented hashing algorithms
/// ```
/// use std::str::FromStr;
/// use hasher::algorithms::{HashAlgorithm, HashFrom};
///
/// let algorithm = HashAlgorithm::from_str("SHA256").unwrap();
/// let digest = algorithm.hash("hello world");
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HashAlgorithm {
    MD5,
    SHA256,
    SHA512
}

impl FromStr for HashAlgorithm {
    type Err = HashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MD5" | "md5" => Ok(HashAlgorithm::MD5),
            "SHA256" | "SHA-256" | "sha256" | "sha-256" => Ok(HashAlgorithm::SHA256),
            "SHA512" | "SHA-512" | "sha512" | "sha-512" => Ok(HashAlgorithm::SHA512),
            _ => Err(HashError::AlgorithmNotFound)
        }
    }
}

/// The `HashFrom<T>` trait for presenting different types of data
pub trait HashFrom<T> {
    fn hash(&self, source: T) -> Result<String, HashError>;
}

impl HashFrom<&[u8]> for HashAlgorithm {
    fn hash(&self, source: &[u8]) -> Result<String, HashError> {
        match self {
            HashAlgorithm::MD5 => md::md5(source),
            HashAlgorithm::SHA256 => sha2::sha256(source),
            HashAlgorithm::SHA512 => sha2::sha512(source)
        }
    }
}

impl HashFrom<&str> for HashAlgorithm {
    fn hash(&self, source: &str) -> Result<String, HashError> {
        self.hash(source.as_bytes())
    }
}

impl HashFrom<String> for HashAlgorithm {
    fn hash(&self, source: String) -> Result<String, HashError> {
        self.hash(source.as_bytes())
    }
}