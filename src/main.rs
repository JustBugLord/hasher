use std::path::Path;
use std::str::FromStr;
use hasher::algorithms::*;

fn main() {
    // use std::time::Instant;
    // let mut average: u128 = 0;
    // for _i in 0..10000 {
    //     let before = Instant::now();
        // let _hasherd = HashAlgorithm::SHA256.digest(String::from("hello_world"));
        // let _hashed = HashAlgorithm::SHA512.hash("hello world");
        // average += before.elapsed().as_nanos();
    // }
    // println!("Average time: {} ns", average / 10000);

    let algorithm = HashAlgorithm::from_str("sha256").unwrap();
    let digest = algorithm.hash(Path::new("D:\\Projects\\Rust2025\\RustroverProjects\\hasher\\src"));
    println!("Digest: {:?}", digest)
}
