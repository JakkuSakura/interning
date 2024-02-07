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
fn bench_new_string(words: &Vec<String>, out: &mut Vec<String>) {
    for word in words {
        let s = word.clone();
        out.push(s);
    }
}
fn bench_insert_hashmap_hash_to_str<'a>(
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
fn bench_lookup_str_from_string(words: &[String]) -> u64 {
    words.iter().map(|s| sum_str(s)).sum()
}
fn bench_hashmap_lookup_str_from_hash(
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

fn bench_hashmap_lookup_id(map: &HashMap<String, i64>, words: &[String]) -> i64 {
    words.iter().map(|s| *map.get(s).unwrap()).sum()
}
fn bench_interned_lookup_id(strs: &[InternedString]) -> i64 {
    strs.iter().map(|s| s.hash().hash() as i64).sum()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let words = words();
    c.benchmark_group("setup_to_str_mapping")
        .bench_function("new_str", |b| {
            b.iter_with_large_drop(|| {
                let mut out = Vec::new();
                bench_new_string(&words, &mut out);
                out
            })
        })
        .bench_function("insert_hashmap_hash_to_str", |b| {
            b.iter_custom(|iters| {
                let mut map = HashMap::new();
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_insert_hashmap_hash_to_str(&mut map, &words);
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
        .bench_function("lookup_str_from_string", |b| {
            b.iter(|| {
                bench_lookup_str_from_string(&words);
            })
        })
        .bench_function("hashmap_lookup_str_from_hash", |b| {
            let hash = words
                .iter()
                .map(|x| InternedStringHash::from_str(x))
                .collect::<Vec<_>>();
            b.iter_custom(|iters| {
                let mut map = HashMap::new();
                bench_insert_hashmap_hash_to_str(&mut map, &words);
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_hashmap_lookup_str_from_hash(&mut map, &hash);
                }
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
    c.benchmark_group("lookup_id")
        .bench_function("hashmap_lookup_id_from_str", |b| {
            let map = words
                .iter()
                .enumerate()
                .map(|(i, x)| (x.clone(), i as i64))
                .collect::<HashMap<_, _>>();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_hashmap_lookup_id(&map, &words);
                }
                start.elapsed()
            })
        })
        .bench_function("lookup_interned_id", |b| {
            let strs = words
                .iter()
                .map(|x| InternedString::from_str(x))
                .collect::<Vec<_>>();
            b.iter_custom(|iters| {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    bench_interned_lookup_id(&strs);
                }
                start.elapsed()
            })
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
