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
use criterion::{criterion_group, criterion_main, Criterion};
use tuid::gen;

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "tuid-rs/approx")]
    tuid::time::updater(1000 * 10).start().unwrap();
    let mut grp = c.benchmark_group("encoding");

    grp.bench_function("hex (encode)", |b| {
        let rng = fastrand::Rng::default();
        b.iter(|| gen::default(rng.u64(..), rng.u64(..)).as_hex());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
