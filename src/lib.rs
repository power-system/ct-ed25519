#![no_std]
#![forbid(unsafe_code)]

mod error;
mod field;
mod keys;
mod point;
mod scalar;
mod sign;
mod signature;
mod verify;

pub use error::SignatureError;
pub use keys::{SigningKey, VerifyingKey};
pub use signature::Signature;

pub use ::signature::{Signer, Verifier};
