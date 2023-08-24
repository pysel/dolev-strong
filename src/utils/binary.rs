pub fn bytes_to_decimal(bz: Vec<u8>) -> i32 {
    let mut reverse_counter: u32 = (bz.len()).try_into().unwrap();
    let mut result: i32 = 0;
    for i in 0..bz.len() {
        reverse_counter -= 1;

        let k: i32 = bz[i].try_into().unwrap();
        let pow2: i32 = 2_i32.pow(reverse_counter);
        let mult: i32 = k * pow2;
        result += mult;
    }
    result
}

/* 
[0, 0, 1, 1] in binary is 3 in decimal

Conversion process:

0 * (2 ^ 3) + 0 * (2 ^ 2) + 1 * (2 ^ 1) + 1 * (2 ^ 0)
*/

mod tests {
    use super::bytes_to_decimal;

    #[test]
    fn test_bytes_to_i32() {
        #[derive(Clone)]
        struct Test {
            bz: Vec<u8>,
            expected_result: i32,
        }

        let tests: Vec<Test> = vec![
            Test{
                bz: [0, 0, 0, 1].to_vec(),
                expected_result: 1
            },

            Test{
                bz: [0, 0, 1, 1].to_vec(),
                expected_result: 3
            },

            Test{
                bz: [1,1,0,0,1,0,0].to_vec(),
                expected_result: 100
            },

            Test{
                bz: [1,1,0,0,1,1].to_vec(),
                expected_result: 51
            },
        ];

        for test in tests {
            let result: i32 = bytes_to_decimal(test.bz);
            assert_eq!(result, test.expected_result)
        }
    }
}