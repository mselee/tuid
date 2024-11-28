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
#![forbid(unsafe_code)]
pub mod config;
mod core;
#[cfg(feature = "fastrand")]
pub mod generator;
pub mod time;

#[cfg(feature = "uuid")]
pub use uuid;

pub use core::{default, new, once, unchecked};

pub type Bytes = [u8; 16];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Tuid(Bytes);

impl Tuid {
    pub fn as_bytes(&self) -> &Bytes {
        &self.0
    }
}

#[cfg(feature = "hex")]
impl Tuid {
    #[inline]
    pub fn as_hex(&self) -> Result<bstr::BString, faster_hex::Error> {
        let mut bytes: [u8; 32] = [0; 32];
        faster_hex::hex_encode(&self.0, &mut bytes)?;
        Ok(bstr::BString::from(bytes))
    }

    #[inline]
    pub fn from_hex<'a, T: Into<&'a [u8]>>(value: T) -> Result<Self, faster_hex::Error> {
        let mut bytes = Bytes::default();
        faster_hex::hex_decode(value.into(), &mut bytes)?;
        Ok(bytes.into())
    }
}

#[cfg(feature = "uuid")]
impl Tuid {
    #[inline]
    pub fn as_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.0)
    }

    #[inline]
    pub fn from_uuid(value: uuid::Uuid) -> Self {
        value.into()
    }
}

#[cfg(feature = "uuid")]
impl From<Tuid> for uuid::Uuid {
    #[inline]
    fn from(value: Tuid) -> Self {
        value.as_uuid()
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for Tuid {
    #[inline]
    fn from(value: uuid::Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl From<Bytes> for Tuid {
    #[inline]
    fn from(bytes: Bytes) -> Self {
        Tuid(bytes)
    }
}

impl From<Tuid> for Bytes {
    #[inline]
    fn from(tuid: Tuid) -> Self {
        tuid.0
    }
}
