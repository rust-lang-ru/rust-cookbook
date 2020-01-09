## Параллельная проверка элементов коллекции на соответствие предикату

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

Этот пример демонстрирует методы [`rayon::any`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.any) и [`rayon::all`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.all), которые являются параллельными аналогами [`std::any`] и [`std::all`] соответственно.
Метод [`rayon::any`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.any) параллельно проверяет, выполняется ли предикат для какого-нибудь элемента итератора и, если да, то как можно раньше возвращает такой элемент.
Метод [`rayon::all`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.all) параллельно проверяет, выполняется ли предикат для всех элементов итератора, и, если нет, то как можно раньше возвращает элемент, нарушающий условие предиката.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8 ));
    assert!(vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8 ));
    assert!(!vec.par_iter().all(|n| *n <= 8 )); 
}
```


[`std::all`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.all
[`std::any`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.any