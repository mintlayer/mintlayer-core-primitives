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

use crate::Destination;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoinType {
    Mainnet,
    Testnet,
    Regtest,
    Signet,
}

impl CoinType {
    pub const fn coin_ticker(&self) -> &'static str {
        match self {
            Self::Mainnet => "ML",
            Self::Testnet => "TML",
            Self::Regtest => "RML",
            Self::Signet => "SML",
        }
    }

    pub const fn bip44_coin_type(&self) -> u32 {
        let hardened_bit = 1 << 31;
        match self {
            Self::Mainnet => 19788 + hardened_bit,
            Self::Testnet | Self::Regtest | Self::Signet => 1 + hardened_bit,
        }
    }

    pub const fn coin_decimals(&self) -> u8 {
        11
    }

    pub const fn address_prefix(&self, destination: &Destination) -> &'static str {
        match self {
            Self::Mainnet => match destination {
                Destination::AnyoneCanSpend => "mxanyonecanspend",
                Destination::PublicKeyHash(_) => "mtc",
                Destination::PublicKey(_) => "mptc",
                Destination::ScriptHash(_) => "mstc",
                Destination::ClassicMultisig(_) => "mmtc",
            },
            Self::Testnet => match destination {
                Destination::AnyoneCanSpend => "txanyonecanspend",
                Destination::PublicKeyHash(_) => "tmt",
                Destination::PublicKey(_) => "tpmt",
                Destination::ScriptHash(_) => "tstc",
                Destination::ClassicMultisig(_) => "tmtc",
            },
            Self::Regtest => match destination {
                Destination::AnyoneCanSpend => "rxanyonecanspend",
                Destination::PublicKeyHash(_) => "rmt",
                Destination::PublicKey(_) => "rpmt",
                Destination::ScriptHash(_) => "rstc",
                Destination::ClassicMultisig(_) => "rmtc",
            },
            Self::Signet => match destination {
                Destination::AnyoneCanSpend => "sxanyonecanspend",
                Destination::PublicKeyHash(_) => "smt",
                Destination::PublicKey(_) => "spmt",
                Destination::ScriptHash(_) => "sstc",
                Destination::ClassicMultisig(_) => "smtc",
            },
        }
    }

    pub const fn pool_id_address_prefix(&self) -> &'static str {
        match self {
            Self::Mainnet => "mpool",
            Self::Testnet => "tpool",
            Self::Regtest => "rpool",
            Self::Signet => "spool",
        }
    }

    pub const fn delegation_id_address_prefix(&self) -> &'static str {
        match self {
            Self::Mainnet => "mdelg",
            Self::Testnet => "tdelg",
            Self::Regtest => "rdelg",
            Self::Signet => "sdelg",
        }
    }

    pub const fn token_id_address_prefix(&self) -> &'static str {
        match self {
            Self::Mainnet => "mmltk",
            Self::Testnet => "tmltk",
            Self::Regtest => "rmltk",
            Self::Signet => "smltk",
        }
    }

    pub const fn order_id_address_prefix(&self) -> &'static str {
        match self {
            Self::Mainnet => "mordr",
            Self::Testnet => "tordr",
            Self::Regtest => "rordr",
            Self::Signet => "sordr",
        }
    }

    pub const fn vrf_public_key_address_prefix(&self) -> &'static str {
        match self {
            Self::Mainnet => "mvrfpk",
            Self::Testnet => "tvrfpk",
            Self::Regtest => "rvrfpk",
            Self::Signet => "svrfpk",
        }
    }
}
