use arrayvec::ArrayString;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use interning::lookup::{global_cleanup, local_cleanup};
use interning::{InternedString, InternedStringHash};
use std::collections::HashMap;

fn words() -> Vec<String> {
    (0..100).map(|x| format!("{}", x)).collect()
}
fn sum_str(s: &str) -> u64 {
    s.chars().map(|x| x as u64).sum()
}
fn bench_insert_hashmap_id_to_str<'a>(
    map: &mut HashMap<InternedStringHash, &'a str>,
    words: &'a Vec<String>,
) {
    map.clear();
    for word in words {
        let hash = InternedStringHash::from_str(word);
        map.insert(hash, word);
    }
}
fn bench_insert_interned(words: &Vec<String>) {
    local_cleanup();
    global_cleanup();
    for word in words {
        let s = InternedString::from_str(word);
        black_box(s);
    }
}
fn bench_new_array_string<const N: usize>(words: &Vec<String>) {
    for word in words {
        let s = ArrayString::<N>::from(word).unwrap();
        black_box(s);
    }
}
fn bench_hashmap_lookup_str(
    map: &mut HashMap<InternedStringHash, &str>,
    hash: &[InternedStringHash],
) -> u64 {
    hash.iter().map(|h| sum_str(map.get(h).unwrap())).sum()
}
fn bench_interned_lookup_str(strs: &[InternedString]) -> u64 {
    strs.iter().map(|s| sum_str(s.as_str())).sum()
}
fn bench_array_string_lookup_str<const N: usize>(words: &[ArrayString<N>]) -> u64 {
    words.iter().map(|s| sum_str(s.as_str())).sum()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let words = words();
    c.benchmark_group("setup_to_str_mapping")
        .bench_function("insert_hashmap", |b| {
            b.iter_custom(|iters| {
                let mut map = HashMap::new();
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_insert_hashmap_id_to_str(&mut map, &words);
                }
                start.elapsed()
            })
        })
        .bench_function("insert_interned", |b| {
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_insert_interned(&words);
                }
                start.elapsed()
            })
        })
        .bench_function("new_array_string_15", |b| {
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_new_array_string::<15>(&words);
                }
                start.elapsed()
            });
        })
        .bench_function("new_array_string_31", |b| {
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_new_array_string::<31>(&words);
                }
                start.elapsed()
            });
        });
    c.benchmark_group("lookup_str")
        .bench_function("hashmap_lookup_str", |b| {
            let hash = words
                .iter()
                .map(|x| InternedStringHash::from_str(x))
                .collect::<Vec<_>>();
            b.iter_custom(|iters| {
                let mut map = HashMap::new();
                bench_insert_hashmap_id_to_str(&mut map, &words);
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_hashmap_lookup_str(&mut map, &hash);
                }
                bench_hashmap_lookup_str(&mut map, &hash);
                start.elapsed()
            })
        })
        .bench_function("lookup_interned", |b| {
            bench_insert_interned(&words);
            let strs = words
                .iter()
                .map(|x| InternedString::from_str(x))
                .collect::<Vec<_>>();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_interned_lookup_str(&strs);
                }
                start.elapsed()
            })
        })
        .bench_function("lookup_array_string_15", |b| {
            let words = words
                .iter()
                .map(|x| ArrayString::<15>::from(x).unwrap())
                .collect::<Vec<_>>();
            b.iter(|| {
                bench_array_string_lookup_str(&words);
            })
        })
        .bench_function("lookup_array_string_31", |b| {
            let words = words
                .iter()
                .map(|x| ArrayString::<31>::from(x).unwrap())
                .collect::<Vec<_>>();
            b.iter(|| {
                bench_array_string_lookup_str(&words);
            })
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
