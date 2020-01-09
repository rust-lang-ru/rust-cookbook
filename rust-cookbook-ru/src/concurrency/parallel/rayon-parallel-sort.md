## Параллельная сортировка вектора

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

Этот пример параллельно сортирует вектор строк.

Более детально, пример создаёт вектор пустых строк, а затем, используя `par_iter_mut().for_each`, параллельно заполняет этот вектор случайными строками. Несмотря на то, что [существует множество способов](https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html) для сортировки элементов, [`par_sort_unstable`](https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable) обычно быстрее, чем алгоритмы [стабильной сортировки](https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort).

```rust,ignore
extern crate rand;
extern crate rayon;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

fn main() {
  let mut vec = vec![String::new(); 100_000];
  vec.par_iter_mut().for_each(|p| {
    let mut rng = thread_rng();
    *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
  });
  vec.par_sort_unstable();
}
```


