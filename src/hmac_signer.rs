pub struct HmacSigner {
    secret_key: [u8; 32],
}

impl HmacSigner {
    pub fn new(secret_key: &str) -> Result<Self, &'static str> {
        let mut authtoken_secret_key = [0u8; 32];
        hex::decode_to_slice(secret_key, &mut authtoken_secret_key)
            .map_err(|_| "invalid secret key format, expected a 256-bit hex string")?;

        Ok(Self {
            secret_key: authtoken_secret_key,
        })
    }

    #[inline]
    pub fn validate(&self, content: &[u8], signature: [u8; 32]) -> bool {
        blake3::keyed_hash(&self.secret_key, content) == blake3::Hash::from_bytes(signature)
    }

    #[inline]
    pub fn sign(&self, content: &[u8]) -> [u8; 32] {
        blake3::keyed_hash(&self.secret_key, content)
            .as_bytes()
            .to_owned()
    }
}
