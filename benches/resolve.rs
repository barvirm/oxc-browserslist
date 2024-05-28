use browserslist::{resolve, Opts};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench(c: &mut Criterion) {
    c.bench_function("resolve 'defaults, not dead'", |b| {
        b.iter(|| {
            resolve(
                black_box(vec!["defaults, not dead"]),
                &black_box(Opts::default()),
            )
        })
    });

    c.bench_function("resolve '> 0.5%'", |b| {
        b.iter(|| resolve(black_box(vec!["> 0.5%"]), &black_box(Opts::default())))
    });

    c.bench_function("resolve 'cover 99%'", |b| {
        b.iter(|| resolve(black_box(vec!["cover 99%"]), &black_box(Opts::default())))
    });

    c.bench_function("resolve 'electron >= 10'", |b| {
        b.iter(|| {
            resolve(
                black_box(vec!["electron >= 10"]),
                &black_box(Opts::default()),
            )
        })
    });

    c.bench_function("resolve 'node >= 8'", |b| {
        b.iter(|| resolve(black_box(vec!["node >= 8"]), &black_box(Opts::default())))
    });

    c.bench_function("resolve 'supports es6-module'", |b| {
        b.iter(|| {
            resolve(
                black_box(vec!["supports es6-module"]),
                &black_box(Opts::default()),
            )
        })
    });
}

criterion_group!(browserslist, bench);
criterion_main!(browserslist);
