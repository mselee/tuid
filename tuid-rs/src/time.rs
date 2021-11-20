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
#[cfg(feature = "approx")]
#[inline]
pub fn update() {
    coarsetime::Instant::update();
}

#[cfg(feature = "approx")]
pub fn updater(ms: u64) -> coarsetime::Updater {
    coarsetime::Updater::new(ms)
}

#[cfg(not(feature = "coarse"))]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .wrapping_add(epoch)
}

#[cfg(all(feature = "coarse", not(feature = "approx")))]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    coarsetime::Clock::now_since_epoch().as_secs().wrapping_add(epoch)
}

#[cfg(feature = "approx")]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    coarsetime::Clock::recent_since_epoch().as_secs().wrapping_add(epoch)
}

#[cfg(all(not(feature = "coarse"), not(test)))]
#[inline]
fn now() -> std::time::SystemTime {
    std::time::SystemTime::now()
}

#[cfg(test)]
fn now() -> std::time::SystemTime {
    use std::{ops::Add, time::Duration};

    std::time::SystemTime::UNIX_EPOCH.add(Duration::from_secs(1519211809934 + (60 * 100)))
}
