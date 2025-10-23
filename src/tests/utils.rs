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

// The value that a compact-encoded integer must have for its encoded form to be 2-bytes long,
// 4-bytes long etc.
pub const SCALE_CODEC_COMPACT_ENC_2_BYTE_VAL_START: u16 = 1 << 6;
pub const SCALE_CODEC_COMPACT_ENC_4_BYTE_VAL_START: u32 = 1 << 14;
pub const SCALE_CODEC_COMPACT_ENC_5_BYTE_VAL_START: u32 = 1 << 30;
pub const SCALE_CODEC_COMPACT_ENC_6_BYTE_VAL_START: u64 = 1 << 32;
pub const SCALE_CODEC_COMPACT_ENC_7_BYTE_VAL_START: u64 = 1 << 40;
pub const SCALE_CODEC_COMPACT_ENC_8_BYTE_VAL_START: u64 = 1 << 48;
pub const SCALE_CODEC_COMPACT_ENC_9_BYTE_VAL_START: u64 = 1 << 56;
pub const SCALE_CODEC_COMPACT_ENC_10_BYTE_VAL_START: u128 = 1 << 64;
pub const SCALE_CODEC_COMPACT_ENC_11_BYTE_VAL_START: u128 = 1 << 72;
pub const SCALE_CODEC_COMPACT_ENC_12_BYTE_VAL_START: u128 = 1 << 80;
pub const SCALE_CODEC_COMPACT_ENC_13_BYTE_VAL_START: u128 = 1 << 88;
pub const SCALE_CODEC_COMPACT_ENC_14_BYTE_VAL_START: u128 = 1 << 96;
pub const SCALE_CODEC_COMPACT_ENC_15_BYTE_VAL_START: u128 = 1 << 104;
pub const SCALE_CODEC_COMPACT_ENC_16_BYTE_VAL_START: u128 = 1 << 112;
pub const SCALE_CODEC_COMPACT_ENC_17_BYTE_VAL_START: u128 = 1 << 120;
