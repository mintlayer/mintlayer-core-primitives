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

use crate::{AccountCommand, AccountNonce, AccountOutPoint, OrderAccountCommand, UtxoOutPoint};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(TxInputTag), derive(strum::EnumIter))]
pub enum TxInput {
    #[codec(index = 0)]
    Utxo(UtxoOutPoint),

    #[codec(index = 1)]
    Account(AccountOutPoint),

    #[codec(index = 2)]
    AccountCommand(AccountNonce, AccountCommand),

    #[codec(index = 3)]
    OrderAccountCommand(OrderAccountCommand),
}
