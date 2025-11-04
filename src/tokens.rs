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

use crate::{Amount, Destination, PscVec, PublicKey};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumIter)]
pub enum IsTokenFreezable {
    #[codec(index = 0)]
    No,

    #[codec(index = 1)]
    Yes,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumIter)]
pub enum IsTokenUnfreezable {
    #[codec(index = 0)]
    No,

    #[codec(index = 1)]
    Yes,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(TokenIssuanceTag), derive(strum::EnumIter))]
pub enum TokenIssuance {
    #[codec(index = 1)]
    V1(TokenIssuanceV1),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(TokenTotalSupplyTag), derive(strum::EnumIter))]
pub enum TokenTotalSupply {
    /// Fixed to a certain amount.
    #[codec(index = 0)]
    Fixed(Amount),

    /// Not known in advance but can be locked once at some point in time.
    #[codec(index = 1)]
    Lockable,

    /// Limited only by the Amount data type.
    #[codec(index = 2)]
    Unlimited,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct TokenIssuanceV1 {
    pub token_ticker: PscVec<u8>,
    pub number_of_decimals: u8,
    pub metadata_uri: PscVec<u8>,
    pub total_supply: TokenTotalSupply,
    pub authority: Destination,
    pub is_freezable: IsTokenFreezable,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(NftIssuanceTag), derive(strum::EnumIter))]
pub enum NftIssuance {
    #[codec(index = 0)]
    V0(NftIssuanceV0),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct NftIssuanceV0 {
    pub creator: Option<PublicKey>,
    pub name: PscVec<u8>,
    pub description: PscVec<u8>,
    pub ticker: PscVec<u8>,
    pub icon_uri: PscVec<u8>,
    pub additional_metadata_uri: PscVec<u8>,
    pub media_uri: PscVec<u8>,
    pub media_hash: PscVec<u8>,
}
