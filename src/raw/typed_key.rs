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

//! Defines a key for key-value maps according to BIP-174. This does not include
//! proprietary keys, which are implemented in the other mod,
//! [`proprietary_key`]
