use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_node::block_storage::BlockStorage;

fn block_storage_benchmark(c: &mut Criterion) {
    let data = include_bytes!("code");
    let mut group = c.benchmark_group("block-storage-group");
    group.sample_size(10);
    group.bench_function(
        "Creating and removing 100 files with size 58 Kb",
        move |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async {
                    let block_storage = BlockStorage::new(40000).await.unwrap();
                    let jobs =
                        (0..=black_box(100)).map(|_| block_storage.create_block(black_box(data)));
                    let results = futures::future::join_all(jobs).await;
                    let jobs = results.into_iter().map(|id| block_storage.delete_block(id));
                    futures::future::join_all(jobs).await;
                })
        },
    );
    group.finish();
}

criterion_group!(benches, block_storage_benchmark);
criterion_main!(benches);
