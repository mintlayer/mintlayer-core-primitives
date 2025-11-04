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

use crate::{GenBlockId, TransactionId};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, strum::EnumDiscriminants,
)]
#[strum_discriminants(name(OutPointSourceIdTag), derive(strum::EnumIter))]
pub enum OutPointSourceId {
    #[codec(index = 0)]
    Transaction(TransactionId),

    #[codec(index = 1)]
    BlockReward(GenBlockId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct UtxoOutPoint {
    id: OutPointSourceId,
    index: u32,
}

impl UtxoOutPoint {
    pub fn new(outpoint_source_id: OutPointSourceId, output_index: u32) -> Self {
        UtxoOutPoint {
            id: outpoint_source_id,
            index: output_index,
        }
    }

    pub fn source_id(&self) -> OutPointSourceId {
        self.id.clone()
    }

    pub fn output_index(&self) -> u32 {
        self.index
    }
}
