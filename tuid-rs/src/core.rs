/*
 * Copyright (c) 2021 Mohamed Seleem.
 *
 * This file is part of tuid.
 * See https://github.com/mselee/tuid for further info.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::{
    config::{Configuration, DefaultConfig},
    time::since,
    Tuid,
};

const U64WIDTH: usize = std::mem::size_of::<u64>();

#[cfg(feature = "fastrand")]
#[inline]
pub fn once() -> Tuid {
    default(fastrand::u64(..), fastrand::u64(..))
}

#[inline]
pub fn default(r1: u64, r2: u64) -> Tuid {
    new::<DefaultConfig>(r1, r2)
}

#[inline]
pub fn new<C: Configuration>(r1: u64, r2: u64) -> Tuid {
    unchecked(
        r1,
        r2,
        crate::config::offset::<C>(),
        crate::config::period::<C>(),
        crate::config::prefix::<C>(),
        crate::config::time_mask::<C>(),
        crate::config::rand_mask::<C>(),
    )
}

#[inline]
pub fn unchecked(r1: u64, r2: u64, offset: u64, period: u64, prefix: u8, time_mask: u64, rand_mask: u64) -> Tuid {
    let ts = since(offset) / period;
    let time_bits = ts & time_mask;
    let rand_bits = r1 & rand_mask;
    let r1 = rand_bits | time_bits;
    let pos = prefix as u32;

    (if cfg!(target_endian = "little") {
        gen_le(pos, r1, r2)
    } else {
        gen_be(pos, r1, r2)
    })
    .to_ne_bytes()
    .into()
}

#[inline(always)]
pub const fn gen_le(pos: u32, r1: u64, r2: u64) -> u128 {
    let r1 = (r1.to_be().rotate_left(pos)) as u128;
    let r2 = (r2 as u128) << (U64WIDTH * 8);
    r1 | r2
}

#[inline(always)]
pub const fn gen_be(pos: u32, r1: u64, r2: u64) -> u128 {
    let r1 = r1.rotate_right(pos) as u128;
    let r1 = r1 << (U64WIDTH * 8);
    r1 | r2 as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuid_gen() {
        let mut rng = fastrand::Rng::with_seed(0);
        let x1 = rng.u64(..);
        let x2 = rng.u64(..);
        let id = default(x1, x2);
        assert_eq!(
            &id.0[..crate::config::prefix::<DefaultConfig>() as usize / 8],
            [82, 143]
        );
    }
}
