// Copyright (c) 2024-2025 RBB S.r.l
// opensource@mintlayer.org
// SPDX-License-Identifier: MIT
// Licensed under the MIT License;
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://github.com/mintlayer/mintlayer-core-primitives/blob/master/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use parity_scale_codec::{Decode, Encode};

pub const PUBLIC_KEY_HASH_SIZE: usize = 20;
pub const SECP256K1_PUBLIC_KEY_SIZE: usize = 33;
pub const SCHNORRKEL_PUBLIC_KEY_SIZE: usize = 32;

// Note: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord are already implemented by the macro,
// so no need to derive them.
fixed_hash::construct_fixed_hash! {
    #[derive(Encode, Decode)]
    pub struct PublicKeyHash(PUBLIC_KEY_HASH_SIZE);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct Secp256k1PublicKey(pub [u8; SECP256K1_PUBLIC_KEY_SIZE]);

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, strum::EnumDiscriminants,
)]
#[strum_discriminants(name(PublicKeyTag), derive(strum::EnumIter))]
pub enum PublicKey {
    #[codec(index = 0)]
    Secp256k1Schnorr(Secp256k1PublicKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct SchnorrkelPublicKey(pub [u8; SCHNORRKEL_PUBLIC_KEY_SIZE]);

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, strum::EnumDiscriminants,
)]
#[strum_discriminants(name(VrfPublicKeyTag), derive(strum::EnumIter))]
pub enum VrfPublicKey {
    #[codec(index = 0)]
    Schnorrkel(SchnorrkelPublicKey),
}
