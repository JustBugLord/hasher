use crate::errors::HashError;

pub fn md5(data: &[u8]) -> Result<String, HashError> {
    let mut input_vec: Vec<u8> = data.to_vec();
    let bit_length: u64 = (input_vec.len() as u64) * 8u64;
    input_vec.push(128_u8);

    while (input_vec.len() * 8) % 512 != 448 {
        input_vec.push(0_u8);
    }

    input_vec.extend([
        bit_length as u8,
        (bit_length >> 8) as u8,
        (bit_length >> 16) as u8,
        (bit_length >> 24) as u8,
        (bit_length >> 32) as u8,
        (bit_length >> 40) as u8,
        (bit_length >> 48) as u8,
        (bit_length >> 56) as u8,
    ]);

    let mut word_a = 0x67452301u32;
    let mut word_b = 0xefcdab89u32;
    let mut word_c = 0x98badcfeu32;
    let mut word_d = 0x10325476u32;

    let mut table: Vec<u32> = Vec::new();
    table.push(0x00000000);

    for i in 1..=64 {
        table.push((4294967296.0 * ((i as f64).sin().abs())) as u32);
    }

    for chunk in input_vec.chunks_exact_mut(64) {
        let x = convert_chunk_u8_to_u32(chunk);
        let word_aa = word_a;
        let word_bb = word_b;
        let word_cc = word_c;
        let word_dd = word_d;

        let result = round_one_operations(word_a, word_b, word_c, word_d, &table, &x);
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        let result = round_two_operations(word_a, word_b, word_c, word_d, &table, &x);
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        let result = round_three_operations(word_a, word_b, word_c, word_d, &table, &x);
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        let result = round_four_operations(word_a, word_b, word_c, word_d, &table, &x);
        word_a = result[0];
        word_b = result[1];
        word_c = result[2];
        word_d = result[3];

        word_a = word_a.wrapping_add(word_aa);
        word_b = word_b.wrapping_add(word_bb);
        word_c = word_c.wrapping_add(word_cc);
        word_d = word_d.wrapping_add(word_dd);
    }

    Ok(format!(
        "{:08x}{:08x}{:08x}{:08x}",
        word_a.swap_bytes(),
        word_b.swap_bytes(),
        word_c.swap_bytes(),
        word_d.swap_bytes()
    ))
}

fn f(x: u32, y: u32, z: u32) -> u32 {
    x & y | !x & z
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    x & z | y & !z
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

fn convert_chunk_u8_to_u32(chunk: &mut [u8]) -> Vec<u32> {
    let mut x: Vec<u32> = Vec::new();
    let mut count = 0;
    let mut temporary_vec: Vec<u8> = Vec::new();
    for i in 0..chunk.len() {
        temporary_vec.push(chunk[i]);
        count += 1;
        if count == 4 {
            let temp_arr: [u8; 4] = temporary_vec.clone().try_into()
                .unwrap_or_else(|_v| panic!("Error converting vector to array - sizes don't match"));
            let value = u32::from_be_bytes(temp_arr);
            x.push(value);
            temporary_vec.clear();
            count = 0;
        }
    }
    x
}

fn round_one_operations(
    mut a: u32,
    mut b: u32,
    mut c: u32,
    mut d: u32,
    table: &Vec<u32>,
    x: &Vec<u32>,
) -> [u32; 4] {
    macro_rules! round1 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i: expr ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(f($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round1!(a, b, c, d, 0, 7, 1);
    round1!(d, a, b, c, 1, 12, 2);
    round1!(c, d, a, b, 2, 17, 3);
    round1!(b, c, d, a, 3, 22, 4);

    round1!(a, b, c, d, 4, 7, 5);
    round1!(d, a, b, c, 5, 12, 6);
    round1!(c, d, a, b, 6, 17, 7);
    round1!(b, c, d, a, 7, 22, 8);

    round1!(a, b, c, d, 8, 7, 9);
    round1!(d, a, b, c, 9, 12, 10);
    round1!(c, d, a, b, 10, 17, 11);
    round1!(b, c, d, a, 11, 22, 12);

    round1!(a, b, c, d, 12, 7, 13);
    round1!(d, a, b, c, 13, 12, 14);
    round1!(c, d, a, b, 14, 17, 15);
    round1!(b, c, d, a, 15, 22, 16);

    [a, b, c, d]
}

fn round_two_operations(
    mut a: u32,
    mut b: u32,
    mut c: u32,
    mut d: u32,
    table: &Vec<u32>,
    x: &Vec<u32>,
) -> [u32; 4] {
    macro_rules! round2 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(g($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round2!(a, b, c, d, 1, 5, 17);
    round2!(d, a, b, c, 6, 9, 18);
    round2!(c, d, a, b, 11, 14, 19);
    round2!(b, c, d, a, 0, 20, 20);

    round2!(a, b, c, d, 5, 5, 21);
    round2!(d, a, b, c, 10, 9, 22);
    round2!(c, d, a, b, 15, 14, 23);
    round2!(b, c, d, a, 4, 20, 24);

    round2!(a, b, c, d, 9, 5, 25);
    round2!(d, a, b, c, 14, 9, 26);
    round2!(c, d, a, b, 3, 14, 27);
    round2!(b, c, d, a, 8, 20, 28);

    round2!(a, b, c, d, 13, 5, 29);
    round2!(d, a, b, c, 2, 9, 30);
    round2!(c, d, a, b, 7, 14, 31);
    round2!(b, c, d, a, 12, 20, 32);

    [a, b, c, d]
}

fn round_three_operations(
    mut a: u32,
    mut b: u32,
    mut c: u32,
    mut d: u32,
    table: &Vec<u32>,
    x: &Vec<u32>,
) -> [u32; 4] {
    macro_rules! round3 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr  ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(h($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round3!(a, b, c, d, 5, 4, 33);
    round3!(d, a, b, c, 8, 11, 34);
    round3!(c, d, a, b, 11, 16, 35);
    round3!(b, c, d, a, 14, 23, 36);

    round3!(a, b, c, d, 1, 4, 37);
    round3!(d, a, b, c, 4, 11, 38);
    round3!(c, d, a, b, 7, 16, 39);
    round3!(b, c, d, a, 10, 23, 40);

    round3!(a, b, c, d, 13, 4, 41);
    round3!(d, a, b, c, 0, 11, 42);
    round3!(c, d, a, b, 3, 16, 43);
    round3!(b, c, d, a, 6, 23, 44);

    round3!(a, b, c, d, 9, 4, 45);
    round3!(d, a, b, c, 12, 11, 46);
    round3!(c, d, a, b, 15, 16, 47);
    round3!(b, c, d, a, 2, 23, 48);

    [a, b, c, d]
}

fn round_four_operations(
    mut a: u32,
    mut b: u32,
    mut c: u32,
    mut d: u32,
    table: &Vec<u32>,
    x: &Vec<u32>,
) -> [u32; 4] {
    macro_rules! round4 {
        ( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i:expr ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(i($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round4!(a, b, c, d, 0, 6, 49);
    round4!(d, a, b, c, 7, 10, 50);
    round4!(c, d, a, b, 14, 15, 51);
    round4!(b, c, d, a, 5, 21, 52);

    round4!(a, b, c, d, 12, 6, 53);
    round4!(d, a, b, c, 3, 10, 54);
    round4!(c, d, a, b, 10, 15, 55);
    round4!(b, c, d, a, 1, 21, 56);

    round4!(a, b, c, d, 8, 6, 57);
    round4!(d, a, b, c, 15, 10, 58);
    round4!(c, d, a, b, 6, 15, 59);
    round4!(b, c, d, a, 13, 21, 60);

    round4!(a, b, c, d, 4, 6, 61);
    round4!(d, a, b, c, 11, 10, 62);
    round4!(c, d, a, b, 2, 15, 63);
    round4!(b, c, d, a, 9, 21, 64);

    [a, b, c, d]
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{HashFrom, HashAlgorithm};

    #[test]
    fn test_md5_str() {
        println!("{:?}", HashAlgorithm::MD5.hash("hello world"));
        assert!(HashAlgorithm::MD5.hash("hello world")
            .unwrap().eq("5eb63bbbe01eeed093cb22bb8f5acdc3"))
    }
}