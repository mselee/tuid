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
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tuid::{gen, Tuid};

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "tuid-rs/approx")]
    tuid::time::updater(1000 * 10).start().unwrap();
    let mut grp = c.benchmark_group("rngs");
    grp.throughput(Throughput::Bytes(std::mem::size_of::<Tuid>() as u64));

    grp.bench_function("rand (impl)", |b| {
        use rand::Rng;

        struct Rand {
            rng: rand::rngs::ThreadRng,
        }

        impl Rand {
            #[inline]
            fn tuid(&mut self) -> Tuid {
                let n1 = self.rng.gen();
                let n2 = self.rng.gen();
                gen::default(n1, n2)
            }
        }

        let mut generator = Rand {
            rng: rand::rngs::ThreadRng::default(),
        };

        b.iter(|| generator.tuid());
    });

    grp.bench_function("nanorand (impl)", |b| {
        use nanorand::Rng;

        struct Nano {
            rng: nanorand::wyrand::WyRand,
        }

        impl Nano {
            #[inline]
            fn tuid(&mut self) -> Tuid {
                let n1 = self.rng.generate();
                let n2 = self.rng.generate();
                gen::default(n1, n2)
            }
        }

        let mut generator = Nano {
            rng: nanorand::wyrand::WyRand::new(),
        };
        b.iter(|| generator.tuid());
    });

    grp.bench_function("fastrand (impl)", |b| {
        // let generator = Fast::default();
        struct Fast {
            rng: fastrand::Rng,
        }

        impl Fast {
            #[inline]
            fn tuid(&mut self) -> Tuid {
                let n1 = self.rng.u64(..);
                let n2 = self.rng.u64(..);
                gen::default(n1, n2)
            }
        }

        let mut generator = Fast {
            rng: fastrand::Rng::default(),
        };
        b.iter(|| generator.tuid());
    });

    grp.bench_function("rand (free)", |b| {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        b.iter(|| gen::default(rng.gen(), rng.gen()));
    });

    grp.bench_function("nanorand (free)", |b| {
        use nanorand::Rng;

        let mut rng = nanorand::wyrand::WyRand::default();
        b.iter(|| gen::default(rng.generate(), rng.generate()));
    });

    grp.bench_function("fastrand (free)", |b| {
        let rng = fastrand::Rng::default();
        b.iter(|| gen::default(rng.u64(..), rng.u64(..)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
