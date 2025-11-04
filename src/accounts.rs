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

use crate::{Amount, DelegationId, Destination, IsTokenUnfreezable, OrderId, PscVec, TokenId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct AccountNonce(#[codec(compact)] pub u64);

/// The type that represents withdrawal from an account.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(AccountSpendingTag), derive(strum::EnumIter))]
pub enum AccountSpending {
    #[codec(index = 0)]
    DelegationBalance(DelegationId, Amount),
}

/// A type of OutPoint that represents spending from an account.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct AccountOutPoint {
    pub nonce: AccountNonce,
    pub spending: AccountSpending,
}

/// This represents a command that can be performed on an account.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(AccountCommandTag), derive(strum::EnumIter))]
pub enum AccountCommand {
    /// Create certain amount of tokens and add them to circulating supply.
    #[codec(index = 0)]
    MintTokens(TokenId, Amount),

    /// Take tokens out of circulation. Not the same as Burn because unminting means that certain
    /// amount of tokens is no longer supported by underlying fiat currency, which can only be
    /// done by the authority.
    #[codec(index = 1)]
    UnmintTokens(TokenId),

    /// After supply is locked tokens cannot be minted or unminted ever again.
    /// Works only for Lockable tokens supply.
    #[codec(index = 2)]
    LockTokenSupply(TokenId),

    /// Freezing token forbids any operation with all the tokens (except for optional unfreeze).
    #[codec(index = 3)]
    FreezeToken(TokenId, IsTokenUnfreezable),

    /// By unfreezing token all operations are available for the tokens again.
    #[codec(index = 4)]
    UnfreezeToken(TokenId),

    /// Change the authority who can authorize operations for a token.
    #[codec(index = 5)]
    ChangeTokenAuthority(TokenId, Destination),

    /// Legacy ConcludeOrder command (orders V0).
    #[codec(index = 6)]
    ConcludeOrder(OrderId),

    /// Legacy FillOrder command (orders V0).
    #[codec(index = 7)]
    FillOrder(OrderId, Amount, Destination),

    /// Change token metadata uri.
    #[codec(index = 8)]
    ChangeTokenMetadataUri(TokenId, PscVec<u8>),
}

/// This represents a command that can be performed on an order account (in orders V1).
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(OrderAccountCommandTag), derive(strum::EnumIter))]
pub enum OrderAccountCommand {
    /// Satisfy an order completely or partially.
    /// The second element is the fill amount in the order's "ask" currency.
    #[codec(index = 0)]
    FillOrder(OrderId, Amount),

    /// Freeze an order which effectively forbids any fill operations.
    /// Frozen order can only be concluded.
    /// Only the address specified as `conclude_key` can authorize this command.
    #[codec(index = 1)]
    FreezeOrder(OrderId),

    /// Close an order and withdraw all remaining funds from both give and ask balances.
    /// Only the address specified as `conclude_key` can authorize this command.
    #[codec(index = 2)]
    ConcludeOrder(OrderId),
}
