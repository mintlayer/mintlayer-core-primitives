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

use parity_scale_codec::{Decode, DecodeAll, Encode};

use crate::TokenId;

pub type PscVec<T> = parity_scale_codec::alloc::vec::Vec<T>;

/// The number of parts per thousand. The valid values are in [0, 1000].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct PerThousand(pub u16);

pub type AmountUIntType = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct Amount {
    #[codec(compact)]
    atoms: AmountUIntType,
}

impl Amount {
    pub const ZERO: Self = Self::from_atoms(0);

    pub const fn from_atoms(v: AmountUIntType) -> Self {
        Amount { atoms: v }
    }

    pub const fn into_atoms(&self) -> AmountUIntType {
        self.atoms
    }
}

/// This represents an amount of an asset, which can be either coins or tokens.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, strum::EnumDiscriminants,
)]
#[strum_discriminants(name(OutputValueTag), derive(strum::EnumIter))]
pub enum OutputValue {
    #[codec(index = 0)]
    Coin(Amount),

    // Note: index = 1 corresponds to TokenV0, which only existed in the early days of testnet
    // and never existed on mainnet.
    #[codec(index = 2)]
    TokenV1(TokenId, Amount),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(OutputTimeLockTag), derive(strum::EnumIter))]
pub enum OutputTimeLock {
    #[codec(index = 0)]
    UntilHeight(BlockHeight),

    #[codec(index = 1)]
    UntilTime(BlockTimestamp),

    #[codec(index = 2)]
    ForBlockCount(BlocksCount),

    #[codec(index = 3)]
    ForSeconds(SecondsCount),
}

pub type BlockHeightUIntType = u64;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Encode, Decode)]
pub struct BlockHeight(#[codec(compact)] pub BlockHeightUIntType);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct BlockTimestamp(pub SecondsCount);

pub type BlocksCountUIntType = u64;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Encode, Decode)]
pub struct BlocksCount(#[codec(compact)] pub BlocksCountUIntType);

pub type SecondsCountUIntType = u64;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Encode, Decode)]
pub struct SecondsCount(#[codec(compact)] pub SecondsCountUIntType);

pub fn encode<T: Encode>(t: &T) -> PscVec<u8> {
    t.encode()
}

pub fn encode_to<T: Encode>(t: &T, buf: &mut PscVec<u8>) {
    t.encode_to(buf)
}

pub fn decode_all<T: Decode>(mut bytes: &[u8]) -> Result<T, parity_scale_codec::Error> {
    T::decode_all(&mut bytes)
}

pub fn encode_as_compact<T>(num: T) -> PscVec<u8>
where
    for<'a> parity_scale_codec::CompactRef<'a, T>: Encode + From<&'a T>,
{
    parity_scale_codec::Compact::<T>::encode(&num.into())
}
