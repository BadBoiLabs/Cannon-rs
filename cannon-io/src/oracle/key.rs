#[derive(Debug, Default, Clone, Copy)]
/// Types of preimage oracle keys. See https://github.com/ethereum-optimism/optimism/blob/develop/specs/fault-proof.md#pre-image-key-types
pub enum KeyType {
    /// Local key types are local and context dependent.
    /// E.g. they might be different depending on the details of what is running the program and why
    Local = 1,
    /// keccak256 keys are global and refer to the data by its hash
    #[default]
    Keccak256 = 2,
    /// Generic are also global and can implement different hash functions or other methods of referring to data
    Generic = 3,
    /// Sha256 preimages are useful for retrieving beacon chain SSZ encoded data. Note this is non-standard as is defined by us
    Sha256 = 129,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PreimageKey {
    pub key_type: KeyType,
    pub x: [u8; 31],
}

impl PreimageKey {
    pub fn new(key_type: KeyType, x: [u8; 31]) -> Self {
        Self { key_type, x }
    }

    /// This will panic if using a slice greater than 31 bytes
    pub fn new_local(x: &[u8]) -> Self {
        let mut key_bytes = [0_u8; 31];
        // slice is inserted at the end
        key_bytes[31 - x.len()..31].clone_from_slice(x);
        Self::new(KeyType::Local, key_bytes)
    }

    /// produce a key from 32 byte of a Keccak hash.
    /// The first byte is discarded and replaced by the type byte
    pub fn new_keccak(hash: [u8; 32]) -> Self {
        let mut x = [0; 31];
        x.clone_from_slice(&hash[1..]);
        Self::new(KeyType::Keccak256, x)
    }

    /// produce a key from 32 byte of a Sha256 hash.
    /// The first byte is discarded and replaced by the type byte
    pub fn new_sha256(hash: [u8; 32]) -> Self {
        let mut x = [0; 31];
        x.clone_from_slice(&hash[1..]);
        Self::new(KeyType::Sha256, x)
    }
}

impl From<PreimageKey> for [u8; 32] {
    fn from(key: PreimageKey) -> Self {
        let mut result = [0; 32];
        result[0] = key.key_type as u8;
        result[1..].copy_from_slice(&key.x);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local() {
        let key = PreimageKey::new_local(&[0xff]);
        assert_eq!(
            <[u8; 32]>::from(key),
            [
                0x1_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0xff
            ]
        )
    }
}
