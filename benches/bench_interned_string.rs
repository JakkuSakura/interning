use criterion::{black_box, criterion_group, criterion_main, Criterion};
use interning::hash::stable_hash_string;
use interning::lookup::{global_cleanup, local_cleanup};
use interning::InternedString;
use std::collections::HashMap;

fn words() -> Vec<String> {
    (0..100).map(|x| format!("{}", x)).collect()
}
fn bench_insert_hashmap(map: &mut HashMap<u64, String>, words: Vec<String>) {
    map.clear();
    for word in words {
        let hash = stable_hash_string(&word);
        map.insert(hash, word);
    }
}
fn bench_insert_interned(words: Vec<String>) {
    local_cleanup();
    global_cleanup();
    for word in words {
        let _ = InternedString::new(word);
    }
}
fn bench_hashmap_lookup(map: &mut HashMap<u64, String>, hash: &[u64]) {
    for &h in hash {
        let foo = map.get(&h).unwrap();
        black_box(foo);
    }
}
fn bench_interned_lookup(strs: &[InternedString]) {
    for str in strs {
        let foo = str.as_str();
        black_box(foo);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert_hashmap", |b| {
        b.iter_batched(
            || (HashMap::new(), words()),
            |(mut map, words)| {
                bench_insert_hashmap(&mut map, words);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("insert_interned", |b| {
        b.iter_batched(
            || words(),
            |words| {
                bench_insert_interned(words);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("lookup_hashmap", |b| {
        let mut map = HashMap::new();
        let words = words();
        bench_insert_hashmap(&mut map, words.clone());
        let hash = words
            .iter()
            .map(|x| stable_hash_string(x))
            .collect::<Vec<_>>();
        b.iter(|| {
            bench_hashmap_lookup(&mut map, &hash);
        })
    });
    c.bench_function("lookup_interned", |b| {
        let words = words();
        bench_insert_interned(words.clone());
        let strs = words
            .iter()
            .map(|x| InternedString::from_str(x))
            .collect::<Vec<_>>();
        b.iter(|| {
            bench_interned_lookup(&strs);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
