use std::collections::HashMap;

fn string_has_unique_chars(s: &str) -> bool {
    // 64 bit unsigned integer
    let mut unique_set: u64 = 0;
    let char_a = 'a';

    for c in s.chars() {
        // character subtraction is meaningless in unicode world
        let position = (c as u16 - char_a as u16) as u32;
        if (1 << position) & unique_set != 0 {
            return false;
        }
        unique_set |= 1 << position;
    }
    true
}

// is s1 the permuation of s2
fn string_permutation_match(s1: &str, s2: &str) -> bool {
    let collect_chars = |s: &str| {
        let mut results = HashMap::new();
        for c in s.chars() {
            let count = results.entry(c).or_insert(0);
            *count += 1;
        }
        results
    };

    let s1_chars = collect_chars(s1);
    let s2_chars = collect_chars(s2);

    for s1_key in s1_chars.keys() {
        if !s2_chars.contains_key(s1_key) || s1_chars.get(s1_key).ne(&s2_chars.get(s1_key)) {
            return false;
        }
    }

    // all char in s2 is used in s1
    for s2_key in s2_chars.keys() {
        if !s1_chars.contains_key(s2_key) {
            return false;
        }
    }

    true
}

fn urlify(input: &str) -> String {
    let tokens = input.split_ascii_whitespace();
    tokens.fold("".to_string(), |acc, v| acc + v)
}

fn palindrome(input: &str) -> bool {
    let chars = input.chars().filter(|c| c.is_alphanumeric());
    let rev = chars.clone().rev();
    chars.eq(rev)
}

fn one_char_inserted(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() + 1 {
        return false;
    }

    let mut idx1 = 0;
    let mut idx2 = 0;
    while idx1 < s1.len() && idx2 < s2.len() {
        let c1 = &s1[idx1..=idx1];
        let c2 = &s2[idx2..=idx2];
        if c1.eq(c2) {
            idx2 += 1;
        }
        idx1 += 1;
    }
    return idx1 >= s1.len() - 1 && idx2 == s2.len();
}

fn string_compression(input: &str) -> String {
    let mut counter = HashMap::new();
    for c in input.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    let mut result = String::new();
    for (key, value) in counter.iter() {
        result.push_str(&format!("{}{}", key, value))
    }

    if result.len() >= input.len() {
        return String::from(input);
    }

    result
}

fn rotate_matrix<Matrix: AsRef<[Row]>, Row: AsRef<[u32]>>(matrix: Matrix) -> Vec<Vec<u32>> {
    let row = matrix.as_ref().len();
    let col = matrix.as_ref()[0].as_ref().len();
    let mut result = vec![vec![0u32; col]; row];
    for i in 0..row {
        for j in 0..col {
            result[i][j] = matrix.as_ref()[col - 1 - j].as_ref()[i]
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{rotate_matrix, string_has_unique_chars};

    #[test]
    fn test_rotate_matrix() {
        let matrix = [[1, 2], [3, 4]];
        let result = rotate_matrix(matrix);
        println!("{result:?}");
        println!("{matrix:?}");
    }

    #[test]
    fn test_string_has_unique_chars() {
        assert!(string_has_unique_chars("abcdefg"));
        assert!(!string_has_unique_chars("aa"))
    }
}
