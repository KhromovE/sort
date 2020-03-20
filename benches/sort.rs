use sort::{bubble_sort, shaker_sort, insertion_sort, quick_sort};
use criterion::{criterion_group, criterion_main, Criterion};
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

fn criterion_benchmark(c: &mut Criterion) {
  let mut rng = StepRng::new(2, 13);
  let mut irs = Irs::default();
  
  let sorted_vec = (1..1000).collect::<Vec<i32>>();
  let mut unsorted_vec = sorted_vec.clone();
  
  irs.shuffle(&mut unsorted_vec, &mut rng).ok();

  c.bench_function("bubble sort for unsorted list", |b| {
    b.iter(|| {
      let mut numbers = unsorted_vec.clone();
      bubble_sort(&mut numbers);
    });
  });

  c.bench_function("bubble sort for sorted list", |b| {
    b.iter(|| {
      let mut numbers = sorted_vec.clone();
      bubble_sort(&mut numbers);
    });
  });

  c.bench_function("shaker sort for unsorted list", |b| {
    b.iter(|| {
      let mut numbers = unsorted_vec.clone();
      shaker_sort(&mut numbers);
    });
  });

  c.bench_function("shaker sort for sorted list", |b| {
    b.iter(|| {
      let mut numbers = sorted_vec.clone();
      shaker_sort(&mut numbers);
    });
  });

  c.bench_function("insertion sort for unsorted list", |b| {
    b.iter(|| {
      let mut numbers = unsorted_vec.clone();
      insertion_sort(&mut numbers);
    });
  });

  c.bench_function("insertion sort for sorted list", |b| {
    b.iter(|| {
      let mut numbers = sorted_vec.clone();
      insertion_sort(&mut numbers);
    });
  });

  c.bench_function("quick sort for unsorted list", |b| {
    b.iter(|| {
      let mut numbers = unsorted_vec.clone();
      quick_sort(&mut numbers);
    });
  });

  c.bench_function("quick sort for sorted list", |b| {
    b.iter(|| {
      let mut numbers = sorted_vec.clone();
      quick_sort(&mut numbers);
    });
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);