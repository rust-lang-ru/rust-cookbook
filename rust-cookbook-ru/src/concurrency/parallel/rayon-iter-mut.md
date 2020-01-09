## Параллельное изменение элементов массива

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

Этот пример использует крейт `rayon`, библиотеку с примитивами для параллелизма данных в Rust. `rayon` реализует метод [`par_iter_mut`](https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefMutIterator.html#tymethod.par_iter_mut) для любого параллельного Iterable типа данных. Это особый тип итератора, который может быть потенциально выполняться параллельно.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
```


