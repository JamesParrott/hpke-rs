//! # HPKE Algorithm Identifiers
//!
//! Algorithm definitions for the [`crate::HpkeCrypto`] trait.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error;

/// KEM Modes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u16)]
pub enum KemAlgorithm {
    /// DH KEM on P256
    DhKemP256 = 0x0010,

    /// DH KEM on P384
    DhKemP384 = 0x0011,

    /// DH KEM on P521
    DhKemP521 = 0x0012,

    /// DH KEM on secp256k1
    DhKemK256 = 0x0016,

    /// DH KEM on x25519
    DhKem25519 = 0x0020,

    /// DH KEM on x448
    DhKem448 = 0x0021,

    /// X-WING
    XWingDraft06 = 0x004D,
}

impl core::fmt::Display for KemAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::convert::TryFrom<u16> for KemAlgorithm {
    type Error = error::Error;
    fn try_from(x: u16) -> Result<KemAlgorithm, Self::Error> {
        match x {
            0x0010 => Ok(KemAlgorithm::DhKemP256),
            0x0011 => Ok(KemAlgorithm::DhKemP384),
            0x0012 => Ok(KemAlgorithm::DhKemP521),
            0x0016 => Ok(KemAlgorithm::DhKemK256),
            0x0020 => Ok(KemAlgorithm::DhKem25519),
            0x0021 => Ok(KemAlgorithm::DhKem448),
            0x004D => Ok(KemAlgorithm::XWingDraft06),
            _ => Err(Self::Error::UnknownKemAlgorithm),
        }
    }
}

impl KemAlgorithm {
    /// Get the length of the private key for the KEM in bytes.
    pub const fn private_key_len(&self) -> usize {
        match self {
            KemAlgorithm::DhKemP256 => 32,
            KemAlgorithm::DhKemP384 => 48,
            KemAlgorithm::DhKemP521 => 66,
            KemAlgorithm::DhKemK256 => 32,
            KemAlgorithm::DhKem25519 => 32,
            KemAlgorithm::DhKem448 => 56,
            KemAlgorithm::XWingDraft06 => 32,
        }
    }

    /// Get the length of the shared secret for the KEM in bytes.
    pub const fn shared_secret_len(&self) -> usize {
        match self {
            KemAlgorithm::DhKemP256 => 32,
            KemAlgorithm::DhKemP384 => 48,
            KemAlgorithm::DhKemP521 => 64,
            KemAlgorithm::DhKemK256 => 32,
            KemAlgorithm::DhKem25519 => 32,
            KemAlgorithm::DhKem448 => 64,
            KemAlgorithm::XWingDraft06 => 32,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
/// AEAD types
pub enum AeadAlgorithm {
    /// AES GCM 128
    Aes128Gcm = 0x0001,

    /// AES GCM 256
    Aes256Gcm = 0x0002,

    /// ChaCha20 Poly1305
    ChaCha20Poly1305 = 0x0003,

    /// HPKE Export-only
    HpkeExport = 0xFFFF,
}

impl core::fmt::Display for AeadAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::convert::TryFrom<u16> for AeadAlgorithm {
    type Error = error::Error;
    fn try_from(x: u16) -> Result<AeadAlgorithm, Self::Error> {
        match x {
            0x0001 => Ok(AeadAlgorithm::Aes128Gcm),
            0x0002 => Ok(AeadAlgorithm::Aes256Gcm),
            0x0003 => Ok(AeadAlgorithm::ChaCha20Poly1305),
            0xFFFF => Ok(AeadAlgorithm::HpkeExport),
            _ => Err(Self::Error::UnknownAeadAlgorithm),
        }
    }
}

impl AeadAlgorithm {
    /// Get the tag size of the [`AeadAlgorithm`] in bytes.
    ///
    /// Note that the function returns `0` for unknown lengths such as the
    /// [`AeadAlgorithm::HpkeExport`] type.
    pub const fn tag_length(&self) -> usize {
        match self {
            AeadAlgorithm::Aes128Gcm => 16,
            AeadAlgorithm::Aes256Gcm => 16,
            AeadAlgorithm::ChaCha20Poly1305 => 16,
            AeadAlgorithm::HpkeExport => 0,
        }
    }

    /// Get the key size of the [`AeadAlgorithm`] in bytes.
    ///
    /// Note that the function returns `0` for unknown lengths such as the
    /// [`AeadAlgorithm::HpkeExport`] type.
    pub const fn key_length(&self) -> usize {
        match self {
            AeadAlgorithm::Aes128Gcm => 16,
            AeadAlgorithm::Aes256Gcm => 32,
            AeadAlgorithm::ChaCha20Poly1305 => 32,
            AeadAlgorithm::HpkeExport => 0,
        }
    }

    /// Get the nonce size of the [`AeadAlgorithm`] in bytes.
    ///
    /// Note that the function returns `0` for unknown lengths such as the
    /// [`AeadAlgorithm::HpkeExport`] type.
    ///
    /// Further note that while the AEAD mechanisms generally allow for different
    /// nonce lengths, this HPKE implementation expects the most common nonce size.
    pub const fn nonce_length(&self) -> usize {
        match self {
            AeadAlgorithm::Aes128Gcm => 12,
            AeadAlgorithm::Aes256Gcm => 12,
            AeadAlgorithm::ChaCha20Poly1305 => 12,
            AeadAlgorithm::HpkeExport => 0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
/// KDF types
/// Value are taken from the HPKE RFC (not published yet)
/// TODO: update when HPKE has been published and values have been registered with
///       IANA.
pub enum KdfAlgorithm {
    /// HKDF SHA 256
    HkdfSha256 = 0x0001,

    /// HKDF SHA 384
    HkdfSha384 = 0x0002,

    /// HKDF SHA 512
    HkdfSha512 = 0x0003,
}

impl core::fmt::Display for KdfAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::convert::TryFrom<u16> for KdfAlgorithm {
    type Error = error::Error;
    fn try_from(x: u16) -> Result<KdfAlgorithm, Self::Error> {
        match x {
            0x0001 => Ok(KdfAlgorithm::HkdfSha256),
            0x0002 => Ok(KdfAlgorithm::HkdfSha384),
            0x0003 => Ok(KdfAlgorithm::HkdfSha512),
            _ => Err(Self::Error::UnknownKdfAlgorithm),
        }
    }
}

impl From<KemAlgorithm> for KdfAlgorithm {
    fn from(kem: KemAlgorithm) -> Self {
        match kem {
            KemAlgorithm::DhKemP256 => KdfAlgorithm::HkdfSha256,
            KemAlgorithm::DhKemP384 => KdfAlgorithm::HkdfSha384,
            KemAlgorithm::DhKemP521 => KdfAlgorithm::HkdfSha512,
            KemAlgorithm::DhKemK256 => KdfAlgorithm::HkdfSha256,
            KemAlgorithm::DhKem25519 => KdfAlgorithm::HkdfSha256,
            KemAlgorithm::DhKem448 => KdfAlgorithm::HkdfSha512,
            KemAlgorithm::XWingDraft06 => KdfAlgorithm::HkdfSha512,
        }
    }
}
