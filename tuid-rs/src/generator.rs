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

use fastrand::Rng;

use crate::{
    config::{Configuration, DefaultConfig},
    Tuid,
};

pub struct Generator<C: Configuration> {
    rng: Rng,
    config: PhantomData<C>,
}

impl<C: Configuration> Generator<C> {
    pub fn new(rng: Rng) -> Self {
        Self {
            rng,
            config: PhantomData,
        }
    }
}

impl Default for Generator<DefaultConfig> {
    fn default() -> Self {
        Self::new(Rng::default())
    }
}

impl<C: Configuration> Iterator for Generator<C> {
    type Item = Tuid;

    fn next(&mut self) -> Option<Self::Item> {
        Some(crate::new::<C>(self.rng.u64(..), self.rng.u64(..)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_builder() {
        let mut generator = Generator::default();

        let _ = generator.next();
    }
}
