use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn bench_vcf_cnv(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-vcf-cnv");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let vcf = manifest.join("tests/golden/test_cnv.vcf");
    c.bench_function("rsomics-vcf-cnv golden", |b| {
        b.iter(|| {
            let tmp = TempDir::new().unwrap();
            let out = Command::new(black_box(bin))
                .args([vcf.to_str().unwrap(), "-o", tmp.path().to_str().unwrap()])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_vcf_cnv);
criterion_main!(benches);
