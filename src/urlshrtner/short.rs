use sha2::{Sha256, Digest};

pub fn hash(input: &String) -> String {
    let mut h = Sha256::new();
    h.update(&input);
    let hash_u32 = &h.finalize()[..];

    format!("{:.6}", hex::encode(hash_u32))
}
