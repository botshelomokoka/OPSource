use criterion::{black_box, criterion_group, criterion_main, Criterion};
use anya_enterprise::rate_limiter::{RateLimiter, RateLimitConfig};
use tokio::runtime::Runtime;

pub fn rate_limiter_benchmarks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let config = RateLimitConfig {
        mode: SystemMode::Enterprise,
        // ... other config fields
    };

    let rate_limiter = rt.block_on(async {
        RateLimiter::new(config).await.unwrap()
    });

    let mut group = c.benchmark_group("rate_limiter");
    
    group.bench_function("check_rate_limit", |b| {
        b.iter(|| {
            rt.block_on(async {
                let context = black_box(SecurityContext::new_test());
                rate_limiter.check_rate_limit(&context).await
            })
        })
    });

    group.finish();
}

criterion_group!(benches, rate_limiter_benchmarks);
criterion_main!(benches); 