const BASE62_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE62_RADIX: u64 = 62;
const SHORT_CODE_LENGTH: usize = 7;
const HASH_SEED_BYTES: usize = std::mem::size_of::<u64>();

/// Generate a short code from the input URL.
///
/// The `attempt` parameter is mixed into the hash input so that collision
/// retries produce a different candidate.
pub fn generate(input: &str, attempt: u32) -> String {
    let hash = hash_input(input, attempt);
    encode_base62(hash, SHORT_CODE_LENGTH)
}

fn hash_input(input: &str, attempt: u32) -> u64 {
    let mut data = input.as_bytes().to_vec();
    data.extend_from_slice(&attempt.to_le_bytes());
    let hash = blake3::hash(&data);
    u64::from_le_bytes(hash.as_bytes()[..HASH_SEED_BYTES].try_into().unwrap())
}

fn encode_base62(mut number: u64, length: usize) -> String {
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let idx = (number % BASE62_RADIX) as usize;
        result.push(BASE62_ALPHABET[idx] as char);
        number /= BASE62_RADIX;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_length() {
        let short = generate("https://example.com", 0);
        assert_eq!(short.len(), SHORT_CODE_LENGTH);
    }

    #[test]
    fn test_generate_deterministic() {
        let url = "https://example.com/test";
        let short1 = generate(url, 0);
        let short2 = generate(url, 0);
        assert_eq!(short1, short2);
    }

    #[test]
    fn test_generate_different_inputs() {
        let short1 = generate("https://example.com/1", 0);
        let short2 = generate("https://example.com/2", 0);
        assert_ne!(short1, short2);
    }

    #[test]
    fn test_generate_different_attempts() {
        let short1 = generate("https://example.com", 0);
        let short2 = generate("https://example.com", 1);
        assert_ne!(short1, short2);
    }

    #[test]
    fn test_generate_alphanumeric() {
        let short = generate("https://example.com", 0);
        assert!(short.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
