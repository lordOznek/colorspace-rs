#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use colorspace::*;

fn convert_checker_vspd() {
    for (_, spd) in colorchecker::SPECTRAL.iter() {
        let xyz = spd.to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
        black_box(xyz);
    }
}

fn convert_checker_spd() {
    for (_, spd) in spd::BABELCOLOR.iter() {
        let xyz = spd.to_xyz();
        black_box(xyz);
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
fn convert_checker_spd_avx() {
    for (_, spd) in spd::BABELCOLOR.iter() {
        let xyz = spd::spd_to_xyz_avx(spd);
        black_box(xyz);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("convert checker vspd", |b| b.iter(convert_checker_vspd));
    c.bench_function("convert checker spd", |b| b.iter(convert_checker_spd));
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    c.bench_function("convert checker spd AVX", |b| b.iter(convert_checker_spd_avx));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
