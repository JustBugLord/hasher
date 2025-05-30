use crate::errors::HashError;

mod constants;

pub fn sha256(data: &[u8]) -> Result<String, HashError> {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    let mut m = data.to_vec();
    m.push(0x80);
    if 64 - m.len() % 64 < 8 {
        m.append(&mut vec![0u8; 64 - m.len() & 64])
    }
    m.append(&mut vec![0u8; 64 - m.len() % 64 - 8]);
    m.append(&mut (data.len() as u64 * 8).to_be_bytes().to_vec());
    let blocks = m.chunks_exact(64);

    for block in blocks {
        let mut w: Vec<u32> = block.chunks_exact(4).map(|chunk| {
            u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
        }).collect();
        w.append(&mut vec![0u32; 48]);

        for i in 16..64 {
            let s0 = (w[i - 15].rotate_right(7)) ^ (w[i - 15].rotate_right(18)) ^ (w[i - 15] >> 3);
            let s1 = (w[i - 2].rotate_right(17)) ^ (w[i - 2].rotate_right(19)) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut tmp_h: [u32; 8] = h.clone();

        for i in 0..64 {
            let s1 = (tmp_h[4].rotate_right(6)) ^ (tmp_h[4].rotate_right(11)) ^ (tmp_h[4].rotate_right(25));
            let ch = (tmp_h[4] & tmp_h[5]) ^ (!tmp_h[4] & tmp_h[6]);
            let temp1 = tmp_h[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(constants::K_SHA256[i]).wrapping_add(w[i]);
            let s0 = (tmp_h[0].rotate_right(2)) ^ (tmp_h[0].rotate_right(13)) ^ (tmp_h[0].rotate_right(22));
            let maj = (tmp_h[0] & tmp_h[1]) ^ (tmp_h[0] & tmp_h[2]) ^ (tmp_h[1] & tmp_h[2]);
            let temp2 = s0.wrapping_add(maj);

            tmp_h[7] = tmp_h[6];
            tmp_h[6] = tmp_h[5];
            tmp_h[5] = tmp_h[4];
            tmp_h[4] = tmp_h[3].wrapping_add(temp1);
            tmp_h[3] = tmp_h[2];
            tmp_h[2] = tmp_h[1];
            tmp_h[1] = tmp_h[0];
            tmp_h[0] = temp1.wrapping_add(temp2);
        }

        for i in 0..8 {
            h[i] = h[i].wrapping_add(tmp_h[i]);
        }
    }

    Ok(h.iter()
        .map(|byte| format!("{:08x}", byte))
        .collect::<Vec<String>>()
        .join(""))
}

pub fn sha512(data: &[u8]) -> Result<String, HashError> {
    let mut h: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];

    let mut m = data.to_vec();
    m.push(0x80);
    if 128 - m.len() % 64 < 8 {
        m.append(&mut vec![0u8; 128 - m.len() & 128])
    }
    m.append(&mut vec![0u8; 128 - m.len() % 128 - 8]);
    m.append(&mut (data.len() as u64 * 8).to_be_bytes().to_vec());
    let blocks = m.chunks_exact(128);

    for block in blocks {
        let mut w: Vec<u64> = block.chunks_exact(8).map(|chunk| {
            u64::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]])
        }).collect();
        w.append(&mut vec![0u64; 64]);

        for i in 16..80 {
            let s0 = (w[i - 15].rotate_right(1)) ^ (w[i - 15].rotate_right(8)) ^ (w[i - 15] >> 7);
            let s1 = (w[i - 2].rotate_right(19)) ^ (w[i - 2].rotate_right(61)) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut tmp_h: [u64; 8] = h.clone();

        for i in 0..80 {
            let s1 = (tmp_h[4].rotate_right(14)) ^ (tmp_h[4].rotate_right(18)) ^ (tmp_h[4].rotate_right(41));
            let ch = (tmp_h[4] & tmp_h[5]) ^ (!tmp_h[4] & tmp_h[6]);
            let temp1 = tmp_h[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(constants::K_SHA512[i]).wrapping_add(w[i]);
            let s0 = (tmp_h[0].rotate_right(28)) ^ (tmp_h[0].rotate_right(34)) ^ (tmp_h[0].rotate_right(39));
            let maj = (tmp_h[0] & tmp_h[1]) ^ (tmp_h[0] & tmp_h[2]) ^ (tmp_h[1] & tmp_h[2]);
            let temp2 = s0.wrapping_add(maj);

            tmp_h[7] = tmp_h[6];
            tmp_h[6] = tmp_h[5];
            tmp_h[5] = tmp_h[4];
            tmp_h[4] = tmp_h[3].wrapping_add(temp1);
            tmp_h[3] = tmp_h[2];
            tmp_h[2] = tmp_h[1];
            tmp_h[1] = tmp_h[0];
            tmp_h[0] = temp1.wrapping_add(temp2);
        }

        for i in 0..8 {
            h[i] = h[i].wrapping_add(tmp_h[i]);
        }
    }

    Ok(h.iter()
        .map(|byte| format!("{:016x}", byte))
        .collect::<Vec<String>>()
        .join(""))
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{HashFrom, HashAlgorithm};

    #[test]
    fn test_sha256_str() {
        assert!(HashAlgorithm::SHA256.hash("hello world")
            .unwrap().eq("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"))
    }

    #[test]
    fn test_sha256_str_string() {
        assert!(HashAlgorithm::SHA256.hash(String::from("hello world"))
            .unwrap().eq("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"))
    }

    #[test]
    fn test_sha512() {
        assert!(HashAlgorithm::SHA512.hash("hello world")
            .unwrap().eq("309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"))
    }
}