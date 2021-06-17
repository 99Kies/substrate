// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generated voter bag thresholds.

use super::N_BAGS;

/// Ratio between adjacent bags;
#[cfg(any(test, feature = "std"))]
#[allow(unused)]
pub const CONSTANT_RATIO: f64 = 1.2483305489016119;

/// Upper thresholds for each bag.
pub const THRESHOLDS: [u64; N_BAGS as usize] = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    11,
    13,
    16,
    19,
    23,
    28,
    34,
    42,
    52,
    64,
    79,
    98,
    122,
    152,
    189,
    235,
    293,
    365,
    455,
    567,
    707,
    882,
    1101,
    1374,
    1715,
    2140,
    2671,
    3334,
    4161,
    5194,
    6483,
    8092,
    10101,
    12609,
    15740,
    19648,
    24527,
    30617,
    38220,
    47711,
    59559,
    74349,
    92812,
    115860,
    144631,
    180547,
    225382,
    281351,
    351219,
    438437,
    547314,
    683228,
    852894,
    1064693,
    1329088,
    1659141,
    2071156,
    2585487,
    3227542,
    4029039,
    5029572,
    6278568,
    7837728,
    9784075,
    12213759,
    15246808,
    19033056,
    23759545,
    29659765,
    37025190,
    46219675,
    57697432,
    72025466,
    89911589,
    112239383,
    140111850,
    174905902,
    218340380,
    272560966,
    340246180,
    424739700,
    530215542,
    661884258,
    826250339,
    1031433539,
    1287569995,
    1607312958,
    2006457867,
    2504722650,
    3126721800,
    3903182340,
    4872461752,
    6082442853,
    7592899225,
    9478448057,
    11832236265,
    14770541991,
    18438518791,
    23017366283,
    28733281486,
    35868633049,
    44775910382,
    55895136784,
    69775606782,
    87103021514,
    108733362657,
    135735178289,
    169442369618,
    211520086272,
    264046985399,
    329617918218,
    411472116776,
    513653213392,
    641208997818,
    800440780206,
    999214678517,
    1247350208103,
    1557105369953,
    1943782201171,
    2426482702132,
    3029052483452,
    3781258749319,
    4720260810076,
    5892445768000,
    7355720059940,
    9182370059991,
    11462633057206,
    14309155016159,
    17862555335640,
    22298373506924,
    27835740839511,
    34748205641269,
    43377246621511,
    54149142084871,
    67596028261358,
    84382187063069,
    105336861893959,
    131495222627659,
    164149503440725,
    204912839732087,
    255798957699744,
    319321653273781,
    398618974707429,
    497608243499122,
    621179571745225,
    775437435763184,
    968002239825113,
    1208386767378873,
    1508466116607513,
    1883064335344139,
    2350686735357198,
    2934434062644189,
    3663143684136207,
    4572814165923224,
    5708383617772005,
    7125949654914296,
    8895540644164415,
    11104575135106362,
    13862180373726516,
    17304583234907174,
    21601839888145304,
    26966236644853160,
    33662776992680304,
    42022272880825152,
    52457686971413784,
    65484533171133904,
    81746343238087392,
    102046457525101200,
    127387710335774608,
    159021970366777056,
    198511983555374656,
    247808573395228608,
    309347012448991104,
    386167325851522816,
    482064469848099072,
    601775804251442048,
    751215120036911616,
    937764783138868096,
    1170640426476344320,
    1461346206149632000,
    1824243111658058240,
    2277258404906088192,
    2842771234587226112,
    3548718175673985024,
    4429973308136232448,
    5530071011365192704,
    6903356581082402816,
    8617670910126150656,
    10757701857491230720,
    13429167864681918464,
    18446744073709551615,
];
