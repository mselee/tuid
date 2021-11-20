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
    const __PERIOD_MIN__: u64 = 1;
    const __PERIOD_MAX__: u64 = 4096;
    const __PERIOD__: u64 = 64;
    const __OFFSET__: u64 = 0;
    const __PREFIX__: u8 = 16;

    const __MASKS__: (u64, u64) = make_masks(Self::__PREFIX__);
    const PERIOD: u64 = make_period(Self::__PERIOD__, Self::__PERIOD_MIN__, Self::__PERIOD_MAX__);
    const OFFSET: u64 = Self::__OFFSET__;
    const TIME_MASK: u64 = Self::__MASKS__.0;
    const RAND_MASK: u64 = Self::__MASKS__.1;
    const PREFIX: usize = Self::__PREFIX__ as usize;
    const WRAPAROUND: Duration = Duration::from_secs(Self::PERIOD * 2u64.pow(Self::PREFIX as u32));
}

pub struct DefaultConfig;

impl Configuration for DefaultConfig {}
impl<C: Configuration> Configuration for &C {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wraparound() {
        assert_eq!(DefaultConfig::WRAPAROUND.as_secs(), 4194304);
    }
}
