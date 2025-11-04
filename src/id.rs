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

use core::marker::PhantomData;

use parity_scale_codec::{Decode, Encode};

#[derive(derive_more::Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct Id<Tag> {
    hash: H256,
    #[debug(skip)]
    _pd: PhantomData<Tag>,
}

impl<Tag> Id<Tag> {
    pub fn new(hash: H256) -> Self {
        Self {
            hash,
            _pd: PhantomData,
        }
    }

    pub fn hash(&self) -> &H256 {
        &self.hash
    }
}

// Note: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord are already implemented by the macro,
// so no need to derive them.
fixed_hash::construct_fixed_hash! {
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

// Note: the derives on the tag types below are technically useless, but they allow us to derive
// the same traits for Id itself.

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderIdTag;
pub type OrderId = Id<OrderIdTag>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenIdTag;
pub type TokenId = Id<TokenIdTag>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DelegationIdTag;
pub type DelegationId = Id<DelegationIdTag>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PoolIdTag;
pub type PoolId = Id<PoolIdTag>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionIdTag;
pub type TransactionId = Id<TransactionIdTag>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenBlockIdTag;
pub type GenBlockId = Id<GenBlockIdTag>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScriptIdTag;
pub type ScriptId = Id<ScriptIdTag>;
