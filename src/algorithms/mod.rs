mod sha2;
mod md;

use crate::errors::HashError;
use crate::errors::HashError::FileOpenError;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

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

impl HashFrom<&String> for HashAlgorithm {
    fn hash(&self, source: &String) -> Result<String, HashError> {
        self.hash(source.as_bytes())
    }
}

impl HashFrom<&Path> for HashAlgorithm {
    fn hash(&self, path: &Path) -> Result<String, HashError> {
        if !path.exists() { return Err(HashError::FileNotFound) }
        let mut file_name: String = String::new();
        if path.file_name().is_some() {
            if let Some(f_name) = path.file_name().unwrap().to_str() {
                file_name = f_name.to_string();
            }
        }
        let mut hashes: Vec<String> = Vec::from([file_name]);
        if path.is_file() {
            match File::open(path) {
                Ok(mut source) => {
                    let mut buffer: [u8; 8192] = [0; 8192];
                    loop {
                        match source.read(&mut buffer) {
                            Ok(bytes_len) => if bytes_len == 0 { break },
                            Err(_e) => return Err(HashError::FileReadError),
                        }
                        match self.hash(buffer.as_slice()) {
                            Ok(step_hash) => hashes.push(step_hash),
                            Err(err) => return Err(err),
                        }
                    }
                },
                Err(_e) => return Err(FileOpenError)
            }
        } else {
            match path.read_dir() {
                Ok(source_dir) => {
                    for entry in source_dir {
                        match entry {
                            Ok(dir_entry) => {
                                match self.hash(dir_entry.path().as_path()) {
                                    Ok(hash) => hashes.push(hash),
                                    Err(err) => return Err(err)
                                }
                            },
                            Err(_e) => return Err(HashError::DirEntryError)
                        }
                    }
                },
                Err(_e) => return Err(FileOpenError)
            }
        }
        self.hash(hashes.join("").as_bytes())
    }
}