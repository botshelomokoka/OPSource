use criterion::{black_box, criterion_group, criterion_main, Criterion};
use anya_enterprise::{
    EnterpriseSystem,
    transaction::TransactionProcessor,
    security::SecuritySystem,
};

pub fn transaction_benchmarks(c: &mut Criterion) {
    let system = EnterpriseSystem::new(Default::default()).unwrap();
    
    c.bench_function("process_transaction", |b| {
        b.iter(|| {
            let tx = black_box(generate_test_transaction());
            system.process_transaction(&tx)
        })
    });

    c.bench_function("security_validation", |b| {
        b.iter(|| {
            let tx = black_box(generate_test_transaction());
            system.security.validate_transaction(&tx)
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(10));
    targets = transaction_benchmarks
);
criterion_main!(benches); 