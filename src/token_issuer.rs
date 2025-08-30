use std::{
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as b64e};
use qunet::buffers::{ByteReader, ByteReaderError, ByteWriter};
use thiserror::Error;

use crate::hmac_signer::HmacSigner;

pub struct TokenIssuer {
    signer: HmacSigner,
    token_expiry: Duration,
}

#[derive(Clone)]
pub struct TokenData {
    pub account_id: i32,
    pub user_id: i32,
    pub username: heapless::String<16>,
    pub roles_str: Option<Box<str>>,
}

#[derive(Debug, Error)]
pub enum TokenValidationError {
    #[error("Invalid token format")]
    InvalidFormat,
    #[error("Could not decode base64 in token: {0}")]
    InvalidBase64(#[from] base64::DecodeSliceError),
    #[error("Invalid binary token structure: {0}")]
    InvalidBinary(#[from] ByteReaderError),
    #[error("Unsupported token version: {0}")]
    UnsupportedVersion(u8),
    #[error("Username too long")]
    UsernameTooLong,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Account ID mismatch")]
    AccountMismatch,
    #[error("Token expired")]
    Expired,
}

impl TokenIssuer {
    pub fn new(secret_key: &str, token_expiry: Duration) -> Result<Self, &'static str> {
        Ok(Self {
            signer: HmacSigner::new(secret_key)?,
            token_expiry,
        })
    }

    pub fn validate(&self, token: &str) -> Result<TokenData, TokenValidationError> {
        let (data, sig) = token
            .split_once('.')
            .ok_or(TokenValidationError::InvalidFormat)?;

        let mut data_buf = [0u8; 1024];
        let data_len = b64e.decode_slice(data, &mut data_buf)?;
        let data = &data_buf[..data_len];

        // validate signature
        let mut sig_buf = [0u8; 32];
        if b64e.decode_slice(sig, &mut sig_buf)? != 32 {
            return Err(TokenValidationError::InvalidSignature);
        }

        if !self.signer.validate(data, sig_buf) {
            return Err(TokenValidationError::InvalidSignature);
        }

        // decode the data
        let mut reader = ByteReader::new(data);
        let version = reader.read_u8()?;

        if version != 2 {
            return Err(TokenValidationError::UnsupportedVersion(version));
        }

        let issued_at = reader.read_i64()?;
        let account_id = reader.read_i32()?;
        let user_id = reader.read_i32()?;
        let username = reader.read_string_u8()?;
        let username = heapless::String::from_str(username)
            .map_err(|_| TokenValidationError::UsernameTooLong)?;
        let roles_str = reader.read_string_u16()?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = issued_at + self.token_expiry.as_secs() as i64;

        if now < issued_at || now > expires_at {
            return Err(TokenValidationError::Expired);
        }

        Ok(TokenData {
            account_id,
            user_id,
            username,
            roles_str: if !roles_str.is_empty() {
                Some(roles_str.to_owned().into_boxed_str())
            } else {
                None
            },
        })
    }

    /// Like `validate`, but also checks that the token matches the given account ID.
    pub fn validate_match(
        &self,
        token: &str,
        account_id: i32,
    ) -> Result<TokenData, TokenValidationError> {
        let data = self.validate(token)?;
        if data.account_id != account_id {
            return Err(TokenValidationError::AccountMismatch);
        }

        Ok(data)
    }

    pub fn generate(
        &self,
        account_id: i32,
        user_id: i32,
        username: &str,
        roles_str: &str,
    ) -> String {
        let mut buf = [0u8; 64];
        let mut writer = ByteWriter::new(&mut buf);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        writer.write_u8(2); // version
        writer.write_i64(now); // issued at
        writer.write_i32(account_id);
        writer.write_i32(user_id);
        writer.write_string_u8(username);
        writer.write_string_u16(roles_str);

        let data = writer.written();

        // sign the token
        let mut sig_buf = [0u8; 43]; // 32 / 3 * 4 + (32 % 3) + 1 for some reason??
        let sig_len = b64e
            .encode_slice(self.signer.sign(data), &mut sig_buf)
            .expect("b64 encoded signature must be exactly 42 bytes long");

        assert_eq!(
            sig_len, 43,
            "b64 encoded signature must be exactly 42 bytes long"
        );

        let mut data_buf = [0u8; 128];
        let data_len = b64e
            .encode_slice(data, &mut data_buf)
            .expect("b64 encoded data must fit in 128 bytes");

        format!(
            "{}.{}",
            str::from_utf8(&data_buf[..data_len]).expect("data must be valid UTF-8"),
            str::from_utf8(&sig_buf).expect("signature must be valid UTF-8"),
        )
    }
}
