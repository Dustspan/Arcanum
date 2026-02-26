use rand::Rng;

const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

pub fn generate_uid() -> String {
    let mut rng = rand::thread_rng();
    let mut segment = || -> String {
        (0..4).map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char).collect()
    };
    format!("ARCANUM-{}-{}", segment(), segment())
}

pub fn is_valid_uid(uid: &str) -> bool {
    uid.starts_with("ARCANUM-") && uid.len() == 17
}
