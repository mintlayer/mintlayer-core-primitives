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

use crate::{
    Amount, DelegationId, Destination, NftIssuance, OutputTimeLock, OutputValue, PerThousand,
    PoolId, PscVec, TokenId, TokenIssuance, VrfPublicKey,
};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(TxOutputTag), derive(strum::EnumIter))]
pub enum TxOutput {
    /// Transfer an output value, giving the provided Destination the authority to
    /// spend it (no conditions).
    #[codec(index = 0)]
    Transfer(OutputValue, Destination),

    /// Same as Transfer, but with the condition that the output can only be
    /// spent after some point in time.
    #[codec(index = 1)]
    LockThenTransfer(OutputValue, Destination, OutputTimeLock),

    /// Burn an amount (whether coin or token).
    #[codec(index = 2)]
    Burn(OutputValue),

    /// Output type that is used to create a stake pool.
    #[codec(index = 3)]
    CreateStakePool(PoolId, StakePoolData),

    /// Output type that represents spending of a stake pool output in a block
    /// reward in order to produce a block.
    #[codec(index = 4)]
    ProduceBlockFromStake(Destination, PoolId),

    /// Create a delegation account to a specific pool, defined by its id.
    /// Takes the owner destination, which is the address authorized to withdraw from the delegation.
    #[codec(index = 5)]
    CreateDelegationId(Destination, PoolId),

    /// Transfer an amount to a delegation.
    #[codec(index = 6)]
    DelegateStaking(Amount, DelegationId),

    /// Issues a new fungible token.
    #[codec(index = 7)]
    IssueFungibleToken(TokenIssuance),

    /// Issue an NFT.
    #[codec(index = 8)]
    IssueNft(TokenId, NftIssuance, Destination),

    /// Deposit data into the blockchain.
    #[codec(index = 9)]
    DataDeposit(PscVec<u8>),

    /// Transfer an output value under Hashed TimeLock Contract.
    #[codec(index = 10)]
    Htlc(OutputValue, HashedTimelockContract),

    /// Create an order.
    #[codec(index = 11)]
    CreateOrder(OrderData),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct StakePoolData {
    pub pledge: Amount,
    pub staker: Destination,
    pub vrf_public_key: VrfPublicKey,
    pub decommission_key: Destination,
    pub margin_ratio_per_thousand: PerThousand,
    pub cost_per_block: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct OrderData {
    /// The key that can authorize the conclusion or freezing of the order.
    pub conclude_key: Destination,
    /// The amount of an asset that the order maker wants to receive.
    pub ask: OutputValue,
    /// The amount of an asset that the order maker wants to give.
    pub give: OutputValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct HashedTimelockContract {
    /// The hash of the HTLC secret.
    pub secret_hash: HtlcSecretHash,
    /// The key that can authorize the normal spending of the HTLC output (i.e. when the secret
    /// is provided).
    pub spend_key: Destination,

    /// The timelock after which the HTLC output can be spent via `refund_key`.
    pub refund_timelock: OutputTimeLock,
    /// The key that can authorize the refund of the HTLC.
    pub refund_key: Destination,
}

pub const HTLC_SECRET_HASH_SIZE: usize = 20;

// Note: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord are already implemented by the macro,
// so no need to derive them.
fixed_hash::construct_fixed_hash! {
    #[derive(Encode, Decode)]
    pub struct HtlcSecretHash(HTLC_SECRET_HASH_SIZE);
}
