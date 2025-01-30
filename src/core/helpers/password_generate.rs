use super::merge_interlaced;

pub fn execute(uuid: &str, email: &str) -> String {
    let merged = merge_interlaced::execute(uuid, email);
    return merged;
}

pub fn verify(uuid: &str, email: &str, password: String) -> bool {
    let merged = merge_interlaced::execute(uuid, email);
    return merged == password;
}
