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
#[cfg(feature = "fastrand")]
pub use self::fast::Fast;
use crate::{
    config::{Configuration, DefaultConfig},
    time::since,
    Tuid,
};

#[cfg(feature = "fastrand")]
mod fast;

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
        C::OFFSET,
        C::PERIOD,
        C::PREFIX as u32,
        C::TIME_MASK,
        C::RAND_MASK,
    )
}

#[inline]
pub fn unchecked(r1: u64, r2: u64, offset: u64, period: u64, prefix: u32, time_mask: u64, rand_mask: u64) -> Tuid {
    let ts = since(offset) / period;
    let time_bits = ts & time_mask;
    let rand_bits = r1 & rand_mask;
    let r1 = rand_bits | time_bits;
    let pos = prefix;

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

    fn xkcd() -> Tuid {
        default(4u64, 4u64)
    }

    #[test]
    fn test_tuid_gen() {
        let rng = fastrand::Rng::with_seed(0);
        let id = default(rng.u64(..), rng.u64(..));
        assert_eq!(&id.0[..DefaultConfig::PREFIX / 8], [82, 143]);
    }
}
