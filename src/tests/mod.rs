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

#[cfg(test)]
extern crate std;

mod utils;

use std::prelude::v1::*;

use hex::FromHex;
use parity_scale_codec::Encode;
use strum::IntoEnumIterator as _;

use crate::tests::utils::{
    SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_10_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_11_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_12_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_13_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_14_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_15_BYTE_VAL_START, SCALE_CODEC_COMPACT_ENC_16_BYTE_VAL_START,
    SCALE_CODEC_COMPACT_ENC_17_BYTE_VAL_START,
};

use super::*;

fn hex_encode<T: Encode>(t: &T) -> String {
    hex::encode(t.encode())
}

fn from_hex<T: FromHex>(s: &str) -> T
where
    <T as FromHex>::Error: std::fmt::Debug,
{
    <T as FromHex>::from_hex(s).unwrap()
}

#[test]
fn test_amount_encoding() {
    let val = Amount::from_atoms(0);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "00");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0101");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "02000100");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0300000040");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "070000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0b000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0f00000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "130000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_10_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "17000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_11_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "1b00000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_12_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "1f0000000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_13_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "23000000000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_14_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "2700000000000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_15_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "2b0000000000000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_16_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "2f000000000000000000000000000001");

    let val = Amount::from_atoms(SCALE_CODEC_COMPACT_ENC_17_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "3300000000000000000000000000000001");
}

#[test]
fn test_block_height_encoding() {
    let val = BlockHeight(0);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "00");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0101");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "02000100");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0300000040");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "070000000001");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0b000000000001");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0f00000000000001");

    let val = BlockHeight(SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "130000000000000001");
}

#[test]
fn test_blocks_count_encoding() {
    let val = BlocksCount(0);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "00");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0101");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "02000100");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0300000040");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "070000000001");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0b000000000001");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0f00000000000001");

    let val = BlocksCount(SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "130000000000000001");
}

#[test]
fn test_seconds_count_encoding() {
    let val = SecondsCount(0);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "00");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0101");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "02000100");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0300000040");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "070000000001");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0b000000000001");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0f00000000000001");

    let val = SecondsCount(SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "130000000000000001");
}

#[test]
fn test_block_timestamp_encoding() {
    let val = BlockTimestamp(SecondsCount(0));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "00");

    let val = BlockTimestamp(SecondsCount(
        SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START.into(),
    ));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0101");

    let val = BlockTimestamp(SecondsCount(
        SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START.into(),
    ));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "02000100");

    let val = BlockTimestamp(SecondsCount(
        SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START.into(),
    ));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0300000040");

    let val = BlockTimestamp(SecondsCount(SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "070000000001");

    let val = BlockTimestamp(SecondsCount(SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0b000000000001");

    let val = BlockTimestamp(SecondsCount(SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0f00000000000001");

    let val = BlockTimestamp(SecondsCount(SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "130000000000000001");
}

#[test]
fn test_public_key_hash_encoding() {
    let val = PublicKeyHash(from_hex("1122334455667788990011223344556677889900"));
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "1122334455667788990011223344556677889900");
}

#[test]
fn test_public_key_encoding() {
    for tag in PublicKeyTag::iter() {
        match tag {
            PublicKeyTag::Secp256k1Schnorr => {
                let val = PublicKey::Secp256k1Schnorr(Secp256k1PublicKey(from_hex(
                    "112233445566778899001122334455667788990011223344556677889900112233",
                )));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "00112233445566778899001122334455667788990011223344556677889900112233"
                );
            }
        }
    }
}

#[test]
fn test_vrf_public_key_encoding() {
    for tag in VrfPublicKeyTag::iter() {
        match tag {
            VrfPublicKeyTag::Schnorrkel => {
                let val = VrfPublicKey::Schnorrkel(SchnorrkelPublicKey(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                )));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "001122334455667788990011223344556677889900112233445566778899001122"
                );
            }
        }
    }
}

#[test]
fn test_id_encoding() {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct CustomTag;
    type CustomId = Id<CustomTag>;

    let val = CustomId::new(H256(from_hex(
        "1122334455667788990011223344556677889900112233445566778899001122",
    )));
    let encoded_val = hex_encode(&val);
    assert_eq!(
        encoded_val,
        "1122334455667788990011223344556677889900112233445566778899001122"
    );
}

#[test]
fn test_destination_encoding() {
    for tag in DestinationTag::iter() {
        match tag {
            DestinationTag::AnyoneCanSpend => {
                let val = Destination::AnyoneCanSpend;
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00");
            }
            DestinationTag::PublicKeyHash => {
                let val = Destination::PublicKeyHash(PublicKeyHash(from_hex(
                    "1122334455667788990011223344556677889900",
                )));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "011122334455667788990011223344556677889900");
            }
            DestinationTag::PublicKey => {
                let val = Destination::PublicKey(PublicKey::Secp256k1Schnorr(Secp256k1PublicKey(
                    from_hex("112233445566778899001122334455667788990011223344556677889900112233"),
                )));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "0200112233445566778899001122334455667788990011223344556677889900112233"
                );
            }
            DestinationTag::ScriptHash => {
                let val = Destination::ScriptHash(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "031122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            DestinationTag::ClassicMultisig => {
                let val = Destination::ClassicMultisig(PublicKeyHash(from_hex(
                    "1122334455667788990011223344556677889900",
                )));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "041122334455667788990011223344556677889900");
            }
        }
    }
}

#[test]
fn test_per_thousand_encoding() {
    let val = PerThousand(123);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "7b00");
}

#[test]
fn test_output_value_encoding() {
    for tag in OutputValueTag::iter() {
        match tag {
            OutputValueTag::Coin => {
                let val = OutputValue::Coin(Amount::from_atoms(123));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00ed01");
            }
            OutputValueTag::TokenV1 => {
                let val = OutputValue::TokenV1(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    Amount::from_atoms(123),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "021122334455667788990011223344556677889900112233445566778899001122ed01"
                );
            }
        }
    }
}

#[test]
fn test_account_nonce_encoding() {
    let val = AccountNonce(0);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "00");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0101");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "02000100");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START.into());
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0300000040");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "070000000001");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0b000000000001");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0f00000000000001");

    let val = AccountNonce(SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START);
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "130000000000000001");
}

#[test]
fn test_account_spending_encoding() {
    for tag in AccountSpendingTag::iter() {
        match tag {
            AccountSpendingTag::DelegationBalance => {
                let val = AccountSpending::DelegationBalance(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    Amount::from_atoms(123),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "001122334455667788990011223344556677889900112233445566778899001122ed01"
                );
            }
        }
    }
}

#[test]
fn test_account_outpoint_encoding() {
    let val = AccountOutPoint {
        nonce: AccountNonce(123),
        spending: AccountSpending::DelegationBalance(
            Id::new(H256(from_hex(
                "1122334455667788990011223344556677889900112233445566778899001122",
            ))),
            Amount::from_atoms(123),
        ),
    };
    let encoded_val = hex_encode(&val);
    assert_eq!(
        encoded_val,
        "ed01001122334455667788990011223344556677889900112233445566778899001122ed01"
    );
}

#[test]
fn test_is_token_freezable_encoding() {
    for val in IsTokenFreezable::iter() {
        match val {
            IsTokenFreezable::No => {
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00");
            }
            IsTokenFreezable::Yes => {
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "01");
            }
        }
    }
}

#[test]
fn test_is_token_unfreezable_encoding() {
    for val in IsTokenUnfreezable::iter() {
        match val {
            IsTokenUnfreezable::No => {
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00");
            }
            IsTokenUnfreezable::Yes => {
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "01");
            }
        }
    }
}

#[test]
fn test_account_command_encoding() {
    for tag in AccountCommandTag::iter() {
        match tag {
            AccountCommandTag::MintTokens => {
                let val = AccountCommand::MintTokens(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    Amount::from_atoms(123),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "001122334455667788990011223344556677889900112233445566778899001122ed01"
                );
            }
            AccountCommandTag::UnmintTokens => {
                let val = AccountCommand::UnmintTokens(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "011122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            AccountCommandTag::LockTokenSupply => {
                let val = AccountCommand::LockTokenSupply(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "021122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            AccountCommandTag::FreezeToken => {
                let val = AccountCommand::FreezeToken(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    IsTokenUnfreezable::Yes,
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "03112233445566778899001122334455667788990011223344556677889900112201"
                );
            }
            AccountCommandTag::UnfreezeToken => {
                let val = AccountCommand::UnfreezeToken(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "041122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            AccountCommandTag::ChangeTokenAuthority => {
                let val = AccountCommand::ChangeTokenAuthority(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    Destination::AnyoneCanSpend,
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "05112233445566778899001122334455667788990011223344556677889900112200"
                );
            }
            AccountCommandTag::ConcludeOrder => {
                let val = AccountCommand::ConcludeOrder(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "061122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            AccountCommandTag::FillOrder => {
                let val = AccountCommand::FillOrder(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    Amount::from_atoms(123),
                    Destination::AnyoneCanSpend,
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "071122334455667788990011223344556677889900112233445566778899001122ed0100"
                );
            }
            AccountCommandTag::ChangeTokenMetadataUri => {
                let val = AccountCommand::ChangeTokenMetadataUri(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    from_hex("111122223333"),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "08112233445566778899001122334455667788990011223344556677889900112218111122223333"
                );
            }
        }
    }
}

#[test]
fn test_output_time_lock_encoding() {
    for tag in OutputTimeLockTag::iter() {
        match tag {
            OutputTimeLockTag::UntilHeight => {
                let val = OutputTimeLock::UntilHeight(BlockHeight(123));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00ed01");
            }
            OutputTimeLockTag::UntilTime => {
                let val = OutputTimeLock::UntilTime(BlockTimestamp(SecondsCount(123)));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "01ed01");
            }
            OutputTimeLockTag::ForBlockCount => {
                let val = OutputTimeLock::ForBlockCount(BlocksCount(123));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "02ed01");
            }
            OutputTimeLockTag::ForSeconds => {
                let val = OutputTimeLock::ForSeconds(SecondsCount(123));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "03ed01");
            }
        }
    }
}

#[test]
fn test_order_account_command_encoding() {
    for tag in OrderAccountCommandTag::iter() {
        match tag {
            OrderAccountCommandTag::FillOrder => {
                let val = OrderAccountCommand::FillOrder(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    Amount::from_atoms(123),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "001122334455667788990011223344556677889900112233445566778899001122ed01"
                );
            }
            OrderAccountCommandTag::FreezeOrder => {
                let val = OrderAccountCommand::FreezeOrder(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "011122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            OrderAccountCommandTag::ConcludeOrder => {
                let val = OrderAccountCommand::ConcludeOrder(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "021122334455667788990011223344556677889900112233445566778899001122"
                );
            }
        }
    }
}

#[test]
fn test_stake_pool_data_encoding() {
    let val = StakePoolData {
        pledge: Amount::from_atoms(123),
        staker: Destination::AnyoneCanSpend,
        vrf_public_key: VrfPublicKey::Schnorrkel(SchnorrkelPublicKey(from_hex(
            "1122334455667788990011223344556677889900112233445566778899001122",
        ))),
        decommission_key: Destination::AnyoneCanSpend,
        margin_ratio_per_thousand: PerThousand(123),
        cost_per_block: Amount::from_atoms(123),
    };
    let encoded_val = hex_encode(&val);
    assert_eq!(
        encoded_val,
        "ed0100001122334455667788990011223344556677889900112233445566778899001122007b00ed01"
    );
}

#[test]
fn test_token_total_supply_encoding() {
    for tag in TokenTotalSupplyTag::iter() {
        match tag {
            TokenTotalSupplyTag::Fixed => {
                let val = TokenTotalSupply::Fixed(Amount::from_atoms(123));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00ed01");
            }
            TokenTotalSupplyTag::Lockable => {
                let val = TokenTotalSupply::Lockable;
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "01");
            }
            TokenTotalSupplyTag::Unlimited => {
                let val = TokenTotalSupply::Unlimited;
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "02");
            }
        }
    }
}

#[test]
fn test_token_issuance_encoding() {
    for tag in TokenIssuanceTag::iter() {
        match tag {
            TokenIssuanceTag::V1 => {
                let val = TokenIssuance::V1(TokenIssuanceV1 {
                    token_ticker: from_hex("111122223333"),
                    number_of_decimals: 123,
                    metadata_uri: from_hex("444455556666"),
                    total_supply: TokenTotalSupply::Unlimited,
                    authority: Destination::AnyoneCanSpend,
                    is_freezable: IsTokenFreezable::Yes,
                });
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "01181111222233337b18444455556666020001");
            }
        }
    }
}

#[test]
fn test_nft_issuance_encoding() {
    for tag in NftIssuanceTag::iter() {
        match tag {
            NftIssuanceTag::V0 => {
                let val = NftIssuance::V0(NftIssuanceV0 {
                    creator: Some(PublicKey::Secp256k1Schnorr(Secp256k1PublicKey(from_hex(
                        "112233445566778899001122334455667788990011223344556677889900112233",
                    )))),
                    name: from_hex("1234"),
                    description: from_hex("2345"),
                    ticker: from_hex("3456"),
                    icon_uri: from_hex("4567"),
                    additional_metadata_uri: from_hex("5678"),
                    media_uri: from_hex("6789"),
                    media_hash: from_hex("7890"),
                });
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    concat!(
                        "000100112233445566778899001122334455667788990011223344556677889900112233",
                        "081234082345083456084567085678086789087890"
                    )
                );
            }
        }
    }
}

#[test]
fn test_htlc_encoding() {
    let val = HashedTimelockContract {
        secret_hash: HtlcSecretHash(from_hex("1122334455667788990011223344556677889900")),
        spend_key: Destination::AnyoneCanSpend,
        refund_timelock: OutputTimeLock::ForBlockCount(BlocksCount(123)),
        refund_key: Destination::AnyoneCanSpend,
    };
    let encoded_val = hex_encode(&val);
    assert_eq!(
        encoded_val,
        "11223344556677889900112233445566778899000002ed0100"
    );
}

#[test]
fn test_order_data_encoding() {
    let val = OrderData {
        conclude_key: Destination::AnyoneCanSpend,
        ask: OutputValue::Coin(Amount::from_atoms(123)),
        give: OutputValue::Coin(Amount::from_atoms(234)),
    };
    let encoded_val = hex_encode(&val);
    assert_eq!(encoded_val, "0000ed0100a903");
}

#[test]
fn test_tx_output_encoding() {
    for tag in TxOutputTag::iter() {
        match tag {
            TxOutputTag::Transfer => {
                let val = TxOutput::Transfer(
                    OutputValue::Coin(Amount::from_atoms(123)),
                    Destination::AnyoneCanSpend,
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0000ed0100");
            }
            TxOutputTag::LockThenTransfer => {
                let val = TxOutput::LockThenTransfer(
                    OutputValue::Coin(Amount::from_atoms(123)),
                    Destination::AnyoneCanSpend,
                    OutputTimeLock::UntilHeight(BlockHeight(123)),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0100ed010000ed01");
            }
            TxOutputTag::Burn => {
                let val = TxOutput::Burn(OutputValue::Coin(Amount::from_atoms(123)));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0200ed01");
            }
            TxOutputTag::CreateStakePool => {
                let val = TxOutput::CreateStakePool(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    StakePoolData {
                        pledge: Amount::from_atoms(123),
                        staker: Destination::AnyoneCanSpend,
                        vrf_public_key: VrfPublicKey::Schnorrkel(SchnorrkelPublicKey(from_hex(
                            "1122334455667788990011223344556677889900112233445566778899001122",
                        ))),
                        decommission_key: Destination::AnyoneCanSpend,
                        margin_ratio_per_thousand: PerThousand(123),
                        cost_per_block: Amount::from_atoms(123),
                    },
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    concat!(
                        "031122334455667788990011223344556677889900112233445566778899001122",
                        "ed010000",
                        "1122334455667788990011223344556677889900112233445566778899001122",
                        "007b00ed01"
                    )
                );
            }
            TxOutputTag::ProduceBlockFromStake => {
                let val = TxOutput::ProduceBlockFromStake(
                    Destination::AnyoneCanSpend,
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "04001122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            TxOutputTag::CreateDelegationId => {
                let val = TxOutput::CreateDelegationId(
                    Destination::AnyoneCanSpend,
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "05001122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            TxOutputTag::DelegateStaking => {
                let val = TxOutput::DelegateStaking(
                    Amount::from_atoms(123),
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "06ed011122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            TxOutputTag::IssueFungibleToken => {
                let val = TxOutput::IssueFungibleToken(TokenIssuance::V1(TokenIssuanceV1 {
                    token_ticker: from_hex("111122223333"),
                    number_of_decimals: 123,
                    metadata_uri: from_hex("444455556666"),
                    total_supply: TokenTotalSupply::Unlimited,
                    authority: Destination::AnyoneCanSpend,
                    is_freezable: IsTokenFreezable::Yes,
                }));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0701181111222233337b18444455556666020001");
            }
            TxOutputTag::IssueNft => {
                let val = TxOutput::IssueNft(
                    Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    ))),
                    NftIssuance::V0(NftIssuanceV0 {
                        creator: Some(PublicKey::Secp256k1Schnorr(Secp256k1PublicKey(from_hex(
                            "112233445566778899001122334455667788990011223344556677889900112233",
                        )))),
                        name: from_hex("1234"),
                        description: from_hex("2345"),
                        ticker: from_hex("3456"),
                        icon_uri: from_hex("4567"),
                        additional_metadata_uri: from_hex("5678"),
                        media_uri: from_hex("6789"),
                        media_hash: from_hex("7890"),
                    }),
                    Destination::AnyoneCanSpend,
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    concat!(
                        "081122334455667788990011223344556677889900112233445566778899001122",
                        "000100112233445566778899001122334455667788990011223344556677889900112233",
                        "08123408234508345608456708567808678908789000"
                    )
                );
            }
            TxOutputTag::DataDeposit => {
                let val = TxOutput::DataDeposit(from_hex("1234567890"));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "09141234567890");
            }
            TxOutputTag::Htlc => {
                let val = TxOutput::Htlc(
                    OutputValue::Coin(Amount::from_atoms(123)),
                    HashedTimelockContract {
                        secret_hash: HtlcSecretHash(from_hex(
                            "1122334455667788990011223344556677889900",
                        )),
                        spend_key: Destination::AnyoneCanSpend,
                        refund_timelock: OutputTimeLock::ForBlockCount(BlocksCount(123)),
                        refund_key: Destination::AnyoneCanSpend,
                    },
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "0a00ed0111223344556677889900112233445566778899000002ed0100"
                );
            }
            TxOutputTag::CreateOrder => {
                let val = TxOutput::CreateOrder(OrderData {
                    conclude_key: Destination::AnyoneCanSpend,
                    ask: OutputValue::Coin(Amount::from_atoms(123)),
                    give: OutputValue::Coin(Amount::from_atoms(234)),
                });
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0b0000ed0100a903");
            }
        }
    }
}

#[test]
fn test_sighash_input_commitment_encoding() {
    for tag in SighashInputCommitmentTag::iter() {
        match tag {
            SighashInputCommitmentTag::None => {
                let val = SighashInputCommitment::None;
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "00");
            }
            SighashInputCommitmentTag::Utxo => {
                let val = SighashInputCommitment::Utxo(TxOutput::Transfer(
                    OutputValue::Coin(Amount::from_atoms(123)),
                    Destination::AnyoneCanSpend,
                ));
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "010000ed0100");
            }
            SighashInputCommitmentTag::ProduceBlockFromStakeUtxo => {
                let val = SighashInputCommitment::ProduceBlockFromStakeUtxo {
                    utxo: TxOutput::Transfer(
                        OutputValue::Coin(Amount::from_atoms(123)),
                        Destination::AnyoneCanSpend,
                    ),
                    staker_balance: Amount::from_atoms(123),
                };
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "020000ed0100ed01");
            }
            SighashInputCommitmentTag::FillOrderAccountCommand => {
                let val = SighashInputCommitment::FillOrderAccountCommand {
                    initially_asked: OutputValue::Coin(Amount::from_atoms(123)),
                    initially_given: OutputValue::Coin(Amount::from_atoms(234)),
                };
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0300ed0100a903");
            }
            SighashInputCommitmentTag::ConcludeOrderAccountCommand => {
                let val = SighashInputCommitment::ConcludeOrderAccountCommand {
                    initially_asked: OutputValue::Coin(Amount::from_atoms(123)),
                    initially_given: OutputValue::Coin(Amount::from_atoms(234)),
                    ask_balance: Amount::from_atoms(11),
                    give_balance: Amount::from_atoms(22),
                };
                let encoded_val = hex_encode(&val);
                assert_eq!(encoded_val, "0400ed0100a9032c58");
            }
        }
    }
}

#[test]
fn test_outpoint_source_id_encoding() {
    for tag in OutPointSourceIdTag::iter() {
        match tag {
            OutPointSourceIdTag::Transaction => {
                let val = OutPointSourceId::Transaction(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "001122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            OutPointSourceIdTag::BlockReward => {
                let val = OutPointSourceId::BlockReward(Id::new(H256(from_hex(
                    "1122334455667788990011223344556677889900112233445566778899001122",
                ))));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "011122334455667788990011223344556677889900112233445566778899001122"
                );
            }
        }
    }
}

#[test]
fn test_utxo_outpoint_encoding() {
    let val = UtxoOutPoint::new(
        OutPointSourceId::Transaction(Id::new(H256(from_hex(
            "1122334455667788990011223344556677889900112233445566778899001122",
        )))),
        123,
    );
    let encoded_val = hex_encode(&val);
    assert_eq!(
        encoded_val,
        "0011223344556677889900112233445566778899001122334455667788990011227b000000"
    );
}

#[test]
fn test_tx_input_encoding() {
    for tag in TxInputTag::iter() {
        match tag {
            TxInputTag::Utxo => {
                let val = TxInput::Utxo(UtxoOutPoint::new(
                    OutPointSourceId::Transaction(Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    )))),
                    123,
                ));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "000011223344556677889900112233445566778899001122334455667788990011227b000000"
                );
            }
            TxInputTag::Account => {
                let val = TxInput::Account(AccountOutPoint {
                    nonce: AccountNonce(123),
                    spending: AccountSpending::DelegationBalance(
                        Id::new(H256(from_hex(
                            "1122334455667788990011223344556677889900112233445566778899001122",
                        ))),
                        Amount::from_atoms(123),
                    ),
                });
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "01ed01001122334455667788990011223344556677889900112233445566778899001122ed01"
                );
            }
            TxInputTag::AccountCommand => {
                let val = TxInput::AccountCommand(
                    AccountNonce(123),
                    AccountCommand::UnmintTokens(Id::new(H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    )))),
                );
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "02ed01011122334455667788990011223344556677889900112233445566778899001122"
                );
            }
            TxInputTag::OrderAccountCommand => {
                let val = TxInput::OrderAccountCommand(OrderAccountCommand::FreezeOrder(Id::new(
                    H256(from_hex(
                        "1122334455667788990011223344556677889900112233445566778899001122",
                    )),
                )));
                let encoded_val = hex_encode(&val);
                assert_eq!(
                    encoded_val,
                    "03011122334455667788990011223344556677889900112233445566778899001122"
                );
            }
        }
    }
}
