use crate::core::configs::app_config::APP_CONFIG;

/// Merge Interlaced
///
/// Merge two string skip character every session in string result
/// @param s1 first string: string to merge
/// @param s2 second string: string to merge
/// @param skip skip character: skip number for skip after merge interlaced
///
/// @return string result: after merge and skip string
/// @example
///
/// s1 = "hardy"
/// s2 = "123"
/// skip = 4
///
/// result before skip = "h1a2r3dy"
/// return after skip = "h1ar3d"
pub fn execute(s1: &str, s2: &str) -> String {
    let mut result: Vec<char> = Vec::new();

    let mut i1 = 0;
    let mut i2 = 0;

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    while i1 < chars1.len() && i2 < chars2.len() {
        if result.len() % 2 == 0 {
            result.push(chars1[i1]);
            i1 += 1;
        } else {
            result.push(chars2[i2]);
            i2 += 1;
        }
    }

    while i1 < chars1.len() {
        result.push(chars1[i1]);
        i1 += 1;
    }

    while i2 < chars2.len() {
        result.push(chars2[i2]);
        i2 += 1;
    }

    let res = result.iter().collect::<String>();
    return skip_string(&res, APP_CONFIG.key.skip_number.clone());
}

fn skip_string(s: &str, skip: String) -> String {
    let weights: Vec<char> = skip.as_str().chars().collect();

    let mut result: Vec<char> = Vec::new();
    let mut index = 0;

    for c in s.chars() {
        let m = c as usize * weights[index] as usize;
        let n = (m % 26) + 65;

        let ch = char::from_u32(n as u32).unwrap();
        result.push(ch);

        if index < weights.len() - 1 {
            index += 1;
        } else {
            index = 0;
        }
    }

    result.iter().collect::<String>()
}
