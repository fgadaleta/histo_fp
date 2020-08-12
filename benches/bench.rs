#![feature(test)]

extern crate histo_fp;
extern crate test;

use histo_fp::float::float_type::Float;

#[bench]
fn bench_adding_samples(b: &mut test::Bencher) {
    b.iter(|| {
        let mut h = histo_fp::Histogram::with_buckets(10, None);
        for i in 0..100 {
            h.add_float(Float{number: i as f64});
            h.add_float(Float{number: (i*i) as f64});
            h.add_float(Float{number: (i * i * i) as f64});
        }
        test::black_box(h);
    });
}

#[bench]
fn bench_formatting(b: &mut test::Bencher) {
    use std::string::ToString;

    let mut h = histo_fp::Histogram::with_buckets(10, None);

    for i in 0..100 {
        h.add_float(Float{number: i as f64});
        h.add_float(Float{number: (i*i) as f64});
        h.add_float(Float{number: (i * i * i) as f64});
    }

    b.iter(|| {
        test::black_box(h.to_string());
    });
}
