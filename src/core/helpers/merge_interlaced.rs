pub fn execute(s1: &str, s2: &str) -> String {
    let mut result = String::new();

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

    result
}
