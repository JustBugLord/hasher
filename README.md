# Hasher
## _Just another hasher_

Hasher is a library that implements various hashing algorithms and an application built on the basis of this library.

### Supported algorithms
- SHA256
- SHA512
- MD5

### Features
- Support hashing of files/directories

### Description
All in all, there is nothing to describe here :)
This is a project for self-study.
My main goals for this project are:
- To understand hashing algorithms
- Realize them in a way that would be pleasant to look at

## Building from source

```sh
git clone https://github.com/JustBugLord/hasher.git
cd hasher
cargo build
```

## Binary usage
Default algorithm: SHA256
```sh
hasher.exe -v -a sha512 "hello world"
hasher.exe "/some/path/to/file_or_dir"
hasher.exe -va md5 good
```

## Library usage
```rust
use std::str::FromStr;
use hasher::algorithms::{HashAlgorithm, HashFrom};

fn main() {
    let algorithm = HashAlgorithm::from_str("SHA256").unwrap();
    let _algorithm_alter = HashAlgorithm::SHA256;
    let hash = algorithm.hash("hello world");
    println!("Hash: {:?}", hash)
}
```

Apache License 2.0