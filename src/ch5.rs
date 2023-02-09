fn get_bit(num: i32, index: i32) -> bool {
    (num & (1i32 << index)) != 0i32
}

fn reverse_bit(num: u32, index: u32) -> u32 {
    num | (1u32 << index)
}

fn clear_bit(num: u32, index: u32) -> u32 {
    num & !(1u32 << index)
}

// clear all bit from most significant to i(included)
fn clear_bit_through(num: u32, index: u32) -> u32 {
    num & ((1 << index) - 1)
}

fn update_bit(num: i32, index: i32, bit: bool) -> i32 {
    num & !(1 << index) | ((bit as i32) << index)
}

fn insertion(num_m: i32, num_n: i32, i: u32, j: u32) -> i32 {
    // clear from bit i to bit j in num_m
    num_m & ((1 << i - 1) | !(1 << j - 1)) | (num_n << i)
}

fn pairwise_bit_swap(mut num1: i32) -> i32 {
    let mut i = 0;
    while i < 32 {
        // swap bit i and bit i + 1
        let bit_i = get_bit(num1, i);
        let bit_i1 = get_bit(num1, i + 1);
        num1 = update_bit(num1, i, bit_i1);
        num1 = update_bit(num1, i + 1, bit_i);
        i += 2;
    }
    num1
}
#[cfg(test)]
mod tests {
    use crate::ch5::get_bit;

    use super::{clear_bit, insertion, pairwise_bit_swap};

    #[test]
    fn test_pairwise() {
        let result = pairwise_bit_swap(0b010101);
        println!("{:b}", result);
        assert_eq!(0b101010, pairwise_bit_swap(0b010101));
    }

    #[test]
    fn test_insertion() {
        assert_eq!(0b10001001100, insertion(0b10000000000, 0b10011, 2, 6))
    }

    #[test]
    fn test_get_bit() {
        let num = 0b0110;
        assert_eq!(true, get_bit(num, 1));
        assert_eq!(false, get_bit(num, 0));
        assert_eq!(true, get_bit(num, 2));
    }

    #[test]
    fn test_clear_bit() {
        let num = 0b1111;
        assert_eq!(0b1110, clear_bit(num, 0));
        assert_eq!(0b1101, clear_bit(num, 1));
        assert_eq!(0b1011, clear_bit(num, 2));
    }
}
