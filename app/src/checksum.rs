use sha1_smol::Sha1;

pub fn validate_checksum(checksum: &str, api: &str, params: &str, secret: &str) -> bool {
    let check_string = api.to_string() + params + secret;
    let mut hasher = Sha1::new();
    hasher.update(check_string.as_bytes());
    let check_sum = hasher.digest().to_string();

    check_sum == checksum
}
