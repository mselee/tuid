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
use std::time::Duration;

macro_rules! bitmask {
    ($len:expr) => {
        ((1 << $len) - 1)
    };
}

const fn validate_period(period: u64, min_period: u64, max_period: u64) {
    debug_assert!(min_period > 0, "`min_period` should be more than 0.");
    debug_assert!(
        period >= min_period && period <= max_period,
        "`period` should be within the specified min/max range.",
    );
}

const fn validate_prefix(prefix: u8) {
    debug_assert!(
        prefix >= 1 && prefix <= 48,
        "Time component should not exceed 48 bits, otherwise there isn't enough randomness."
    );
}

#[inline]
const fn make_period(period: u64, min_period: u64, max_period: u64) -> u64 {
    validate_period(period, min_period, max_period);
    period
}

#[inline]
const fn make_masks(prefix: u8) -> (u64, u64) {
    validate_prefix(prefix);
    let start = std::mem::size_of::<u64>() * 8 - prefix as usize;
    let end = prefix as usize;

    let time_mask = bitmask!(end);
    let rand_mask = bitmask!(start) << end;
    (time_mask, rand_mask)
}

pub trait Configuration {
    const PERIOD_MIN: u64 = 1;
    const PERIOD_MAX: u64 = 4096;
    const PERIOD: u64 = 64;
    const OFFSET: u64 = 0;
    const PREFIX: u8 = 16;
}

pub const fn period<C: Configuration>() -> u64 {
    make_period(C::PERIOD, C::PERIOD_MIN, C::PERIOD_MAX)
}

pub const fn masks<C: Configuration>() -> (u64, u64) {
    make_masks(prefix::<C>())
}

pub const fn offset<C: Configuration>() -> u64 {
    C::OFFSET
}

pub const fn time_mask<C: Configuration>() -> u64 {
    masks::<C>().0
}

pub const fn rand_mask<C: Configuration>() -> u64 {
    masks::<C>().1
}

pub const fn prefix<C: Configuration>() -> u8 {
    C::PREFIX
}

pub const fn wraparound<C: Configuration>() -> Duration {
    Duration::from_secs(period::<C>() * 2u64.pow(prefix::<C>() as u32))
}

pub struct DefaultConfig;

impl Configuration for DefaultConfig {}
impl<C: Configuration> Configuration for &C {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wraparound() {
        assert_eq!(wraparound::<DefaultConfig>().as_secs(), 4194304);
    }
}
