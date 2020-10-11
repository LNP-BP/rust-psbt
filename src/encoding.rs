// Rust library for working with partially signed bitcoin transactions (PSBT)
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the Apache License version 2.0 along with
// this software. If not, see <https://opensource.org/licenses/Apache-2.0>.

//! Serialization & deserialization traits encoding PSBT data according to
//! BIP-174

use std::io;

/// Errors from PSBT data structure deserialization
#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Error, From)]
#[display(Debug)]
pub enum DataError {}

/// Encodes PSBT data structure according to BIP-174 rules
pub trait Bip174Encode {
    /// Writes binary data to `writer`
    fn bip174_encode(&self) -> Result<usize, io::Error>;
}

/// Decodes PSBT data structure
pub trait Bip174Decode
where
    Self: Sized,
{
    /// Reads binary data from `reader`
    fn bip174_decode() -> Result<Self, DataError>;
}
