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
    let mut grp = c.benchmark_group("uuid");

    grp.bench_function("generate (raw)", |b| {
        let rng = fastrand::Rng::default();
        b.iter(|| gen::default(rng.u64(..), rng.u64(..)).as_uuid());
    });

    grp.bench_function("generate (string)", |b| {
        let rng = fastrand::Rng::default();
        b.iter(|| gen::default(rng.u64(..), rng.u64(..)).as_uuid().to_string());
    });

    grp.bench_function("generate (simple)", |b| {
        let rng = fastrand::Rng::default();
        b.iter(|| {
            gen::default(rng.u64(..), rng.u64(..))
                .as_uuid()
                .to_simple_ref()
                .to_string()
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
