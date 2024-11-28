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

#[cfg(all(not(test), feature = "approx"))]
pub fn updater(ms: u64) -> coarsetime::Updater {
    coarsetime::Updater::new(ms)
}

#[cfg(all(not(test), not(feature = "coarse"), not(feature = "coarse")))]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .wrapping_add(epoch)
}

#[cfg(all(not(test), feature = "coarse", not(feature = "approx")))]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    coarsetime::Clock::now_since_epoch().as_secs().wrapping_add(epoch)
}

#[cfg(all(not(test), feature = "approx"))]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    coarsetime::Clock::recent_since_epoch().as_secs().wrapping_add(epoch)
}

#[cfg(test)]
#[inline]
pub(crate) fn since(epoch: u64) -> u64 {
    std::time::Duration::from_secs(1519211809934 + (60 * 100))
        .as_secs()
        .wrapping_add(epoch)
}
