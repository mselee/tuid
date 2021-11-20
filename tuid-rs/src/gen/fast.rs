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
use std::marker::PhantomData;

use crate::{
    config::{Configuration, DefaultConfig},
    gen, Tuid,
};

pub struct Fast<C: Configuration> {
    config: PhantomData<C>,
    rng: fastrand::Rng,
}

impl<C: Configuration> Fast<C> {
    pub fn new(rng: fastrand::Rng) -> Self {
        Self {
            config: PhantomData,
            rng,
        }
    }

    #[inline]
    pub fn tuid(&self) -> Tuid {
        gen::new::<C>(self.rng.u64(..), self.rng.u64(..))
    }
}

impl Default for Fast<DefaultConfig> {
    fn default() -> Self {
        let rng = fastrand::Rng::default();
        Self {
            config: PhantomData,
            rng,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_builder() {
        let builder = Fast::default();

        let _ = builder.tuid();
    }
}
