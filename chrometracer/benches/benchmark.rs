use std::time::{Duration, Instant};

use chrometracer::ChromeTracerGuard;
use criterion::{criterion_group, criterion_main, Criterion};
use tempfile::{tempdir, TempDir};

#[chrometracer::instrument(fields(name = test_instrument))]
fn test_instrument() {}

fn test_span() {
    chrometracer::span!(name: "test_span", is_async: false);
}

#[chrometracer::instrument]
fn event_recv(from: Instant) -> Duration {
    Instant::now().duration_since(from)
}

fn initialize_profiler(tmp_dir: &TempDir) -> ChromeTracerGuard {
    chrometracer::builder()
        .trace_file(
            tmp_dir
                .path()
                .join("profile.json")
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .init()
}

fn bench(c: &mut Criterion) {
    let tmp_dir = tempdir().unwrap();
    let _guard = initialize_profiler(&tmp_dir);
    c.bench_function("Profiler Overhead", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;
            for _ in 0..iters {
                duration = duration.saturating_add(event_recv(Instant::now()));
            }
            duration
        })
    });
    c.bench_function("instrument", |b| b.iter(|| test_instrument()));
    c.bench_function("span", |b| b.iter(|| test_span()));
}

criterion_group!(benches, bench);
criterion_main!(benches);
