use aes_gcm::{
    aead::{consts::U12, generic_array::GenericArray},
    Aes256Gcm, Key,
};
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::{
    error::{AppError, AppResult},
    server::CreateGameResponse,
};
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
pub struct Encryptor {
    pub encryption_key: Key<Aes256Gcm>,
    pub nonce: GenericArray<u8, U12>,
}

impl Encryptor {
    pub fn encrypt(&self, plaintext: &[u8]) -> AppResult<Vec<u8>> {
        let cipher = Aes256Gcm::new(&self.encryption_key);
        cipher
            .encrypt(&self.nonce, plaintext)
            .map_err(|e| AppError::custom(format!("Encrypt error: {}", e)))
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> AppResult<Vec<u8>> {
        let cipher = Aes256Gcm::new(&self.encryption_key);
        cipher
            .decrypt(&self.nonce, ciphertext)
            .map_err(|e| AppError::custom(format!("Decrypt error: {}", e)))
    }
}

impl TryFrom<CreateGameResponse> for Encryptor {
    type Error = AppError;

    fn try_from(dto: CreateGameResponse) -> Result<Self, Self::Error> {
        let nonce_bytes: [u8; 12] = STANDARD
            .decode(dto.nonce)
            .map_err(|e| AppError::custom(e.to_string()))?
            .try_into()
            .map_err(|_| AppError::custom("Can't deserialize nonce"))?;

        let nonce = GenericArray::from_slice(&nonce_bytes).to_owned();

        let encryption_key_bytes: [u8; 32] = STANDARD
            .decode(dto.encryption_key)
            .map_err(|e| AppError::custom(e.to_string()))?
            .try_into()
            .map_err(|_| AppError::custom("Can't deserialize encryption key"))?;

        let encryption_key: Key<Aes256Gcm> = encryption_key_bytes.into();

        Ok(Encryptor {
            encryption_key,
            nonce,
        })
    }
}
