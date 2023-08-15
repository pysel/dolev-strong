pub fn bytes_to_i32(bz: Vec<u8>) -> i32 {
    let mut reverse_counter: i32 = (bz.len() - 1).try_into().unwrap();
    let mut result: i32 = 0;
    for i in 0..bz.len() {
        let k: i32 = bz[i].try_into().unwrap();
        let pow2: i32 = 2 ^ reverse_counter;
        let mult: i32 = k * pow2;
        result += mult;
        reverse_counter -= 1;
    }
    result
}

/* 
[0, 0, 1, 1] in binary is 3 in decimal

Conversion process:

0 * (2 ^ 3) + 0 * (2 ^ 2) + 1 * (2 ^ 1) + 1 * (2 ^ 0)
*/