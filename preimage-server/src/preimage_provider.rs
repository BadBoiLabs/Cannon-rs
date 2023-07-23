pub trait PreimageProvider {
    fn get(&self, key: &[u8; 32]) -> Option<Vec<u8>>;
}

impl PreimageProvider for std::collections::HashMap<[u8; 32], Vec<u8>> {
    fn get(&self, key: &[u8; 32]) -> Option<Vec<u8>> {
        self.get(key).cloned()
    }
}
